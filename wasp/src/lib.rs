pub use wasp_core::*;
pub use wasp_http as http;

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        use wasp_core::log::{write, Level};
        use wasp_http::{HttpMessage, HttpResponse, StatusCode};
        let payload = info.payload();
        let location = info
            .location()
            .map(|location| format!("[{}:{}] ", location.file(), location.line()))
            .unwrap_or_else(|| "".to_owned());

        fn send_error<Body>(status: StatusCode, body: Body)
        where
            HttpResponse<Body>: HttpMessage,
        {
            HttpResponse::builder()
                .status(status)
                .body(body)
                .unwrap()
                .send();
        }

        if let Some(status) = payload.downcast_ref::<StatusCode>() {
            return send_error(*status, status.to_string());
        }

        if let Some((status, body)) = payload.downcast_ref::<(StatusCode, Vec<u8>)>() {
            return send_error(*status, body);
        }

        if let Some((status, body)) = payload.downcast_ref::<(StatusCode, String)>() {
            return send_error(*status, body);
        }

        if let Some(err) = payload.downcast_ref::<&str>() {
            write(Level::Error, format!("{}{}", location, err)).unwrap();
            return;
        }

        if let Some(err) = payload.downcast_ref::<String>() {
            write(Level::Error, format!("{}{}", location, err)).unwrap();
            return;
        }

        write(Level::Error, format!("{}Request panicked", location)).unwrap();
    }));
}

pub mod dev {
    pub trait WaspInspect {
        fn inspect(self) -> Self;
    }

    impl<T: std::fmt::Debug> WaspInspect for T {
        fn inspect(self) -> Self {
            inspect(&self);
            self
        }
    }

    pub fn inspect<T: std::fmt::Debug>(data: T) {
        wasp_core::log::write(wasp_core::log::Level::Debug, format!("{:?}", data)).unwrap();
    }
}

pub mod time {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    pub use wasp_core::time::*;

    pub fn system_time() -> SystemTime {
        let (_, secs, nanos) =
            wasp_core::time::os::now().expect("Could not fetch current system time");
        UNIX_EPOCH + Duration::new(secs, nanos)
    }
}
