
use serde_json as json;


fn reply(code: StatusCode) -> StrResult<Response<Body>> {
    Response::builder()
        .status(code)
        .body(Body::empty())
        .map_err(err!())
}

fn reply_json<T: Serialize>(obj: &T) -> StrResult<Response<Body>> {
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(json::to_string(obj).map_err(err!())?.into())
        .map_err(err!())
}

async fn from_request_body<T: DeserializeOwned>(request: Request<Body>) -> StrResult<T> {
    json::from_reader(
        hyper::body::aggregate(request)
            .await
            .map_err(err!())?
            .reader(),
    )
    .map_err(err!())
}


async fn rest_api(
    request: Request<Body>,
    log_sender: broadcast::Sender<String>,
    legacy_events_sender: broadcast::Sender<String>,
    events_sender: broadcast::Sender<Event>,
) -> StrResult<Response<Body>> {
    let mut response = match request.uri().path() {

        "/api/audio-devices" => reply_json(&server_data.read().get_audio_devices_list()?)?,
        other_uri => {
            if other_uri.contains("..") {
                // Attempted tree traversal
                reply(StatusCode::FORBIDDEN)?
            } else {
                let path_branch = match other_uri {
                    "/" => "/index.html",
                    other_path => other_path,
                };

                let maybe_file = tokio::fs::File::open(format!(
                    "{}{path_branch}",
                    FILESYSTEM_LAYOUT.dashboard_dir().to_string_lossy(),
                ))
                .await;

                if let Ok(file) = maybe_file {
                    let mut builder = Response::builder();
                    if other_uri.ends_with(".wasm") {
                        builder = builder.header(CONTENT_TYPE, "application/wasm");
                    }

                    builder
                        .body(Body::wrap_stream(FramedRead::new(file, BytesCodec::new())))
                        .map_err(err!())?
                } else {
                    reply(StatusCode::NOT_FOUND)?
                }
            }
        }
    };

    response.headers_mut().insert(
        CACHE_CONTROL,
        HeaderValue::from_str("no-cache, no-store, must-revalidate").map_err(err!())?,
    );
    response
        .headers_mut()
        .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));

    Ok(response)
}

pub async fn web_server(
    log_sender: broadcast::Sender<String>,
    legacy_events_sender: broadcast::Sender<String>,
    events_sender: broadcast::Sender<Event>,
) -> StrResult {
    let web_server_port = SERVER_DATA_MANAGER
        .read()
        .settings()
        .connection
        .web_server_port;

    let service = service::make_service_fn(|_| {
        let log_sender = log_sender.clone();
        let legacy_events_sender = legacy_events_sender.clone();
        let events_sender = events_sender.clone();
        async move {
            StrResult::Ok(service::service_fn(move |request| {
                let log_sender = log_sender.clone();
                let legacy_events_sender = legacy_events_sender.clone();
                let events_sender = events_sender.clone();
                async move {
                    let res =
                        rest_api(request, log_sender, legacy_events_sender, events_sender).await;
                    if let Err(e) = &res {
                        vors_common::show_e(e);
                    }
                    res
                }
            }))
        }
    });

    hyper::Server::bind(&SocketAddr::new(
        "0.0.0.0".parse().unwrap(),
        web_server_port,
    ))
    .serve(service)
    .await
    .map_err(err!())
}
