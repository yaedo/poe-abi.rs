pub use wasp_core::*;
pub use wasp_http as http;

pub trait WaspAssert<T> {
    fn assert(self, message: &str) -> T;
}

impl<T> WaspAssert<T> for Option<T> {
    fn assert(self, message: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                wasp_core::log::write(wasp_core::log::Level::Error, message).unwrap();
                panic!()
            }
        }
    }
}

impl<T, E: std::fmt::Debug> WaspAssert<T> for Result<T, E> {
    fn assert(self, message: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                wasp_core::log::write(
                    wasp_core::log::Level::Error,
                    &format!("{:?}: {}", e, message),
                )
                .unwrap();
                panic!()
            }
        }
    }
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
        wasp_core::log::write(wasp_core::log::Level::Debug, &format!("{:?}", data)).unwrap();
    }
}
