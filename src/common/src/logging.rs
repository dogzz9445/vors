use std::{backtrace::Backtrace, fmt::Display, future::Future};

// 4 level information green

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let err_str = format!(
            "What happened:\n{panic_info}\n\nBacktrace:\n{}",
            Backtrace::force_capture()
        );

        log::error!("{err_str}");

        #[cfg(windows)]
        std::thread::spawn(move || {
            msgbox::create("ALVR panicked", &err_str, msgbox::IconType::Error).ok();
        });
    }))
}

pub fn show_w<W: Display>(w: W) {
    log::warn!("{w}");

    // GDK crashes because of initialization in multiple thread
    #[cfg(windows)]
    std::thread::spawn({
        let warn_string = w.to_string();
        move || {
            msgbox::create(
                "ALVR encountered a non-fatal error",
                &warn_string,
                msgbox::IconType::Info,
            )
            .ok();
        }
    });
}

pub fn show_warn<T, E: Display>(res: Result<T, E>) -> Option<T> {
    res.map_err(show_w).ok()
}

