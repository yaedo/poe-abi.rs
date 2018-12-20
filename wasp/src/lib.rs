pub use wasp_core::*;
pub use wasp_http as http;

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        use wasp_core::log::{write, Level};
        use wasp_http::{HttpMessage, HttpResponse, StatusCode};
        let payload = info.payload();

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
            write(Level::Error, err).unwrap();
            return;
        }

        write(Level::Error, "Request panicked").unwrap();
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
