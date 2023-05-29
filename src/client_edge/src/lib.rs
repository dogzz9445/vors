mod web_server;

use web_server::*;
use tokio::sync::broadcast::{self, error::RecvError};

fn init() {
    let (events_sender, _) = broadcast::channel(web_server::WS_BROADCAST_CAPACITY);
    logging_backend::init_logging(events_sender.clone());

    if let Some(runtime) = WEBSERVER_RUNTIME.lock().as_mut() {
        runtime.spawn(vors_share_common::show_err_async(web_server::web_server(
            events_sender,
        )));
    }


}
