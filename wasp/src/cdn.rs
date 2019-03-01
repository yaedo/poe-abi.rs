pub use failure::{format_err, Error};
use std::time::{SystemTime, UNIX_EPOCH};
use wasp_core::cdn::sign_protected_object;

pub use wasp_core::cdn::static_url;

pub fn new_protected_object(path: String, expires_at: SystemTime) -> ProtectedObject {
    ProtectedObject::new(path, expires_at)
}

#[derive(Clone)]
pub struct ProtectedObject {
    pub path: String,
    pub expires_at: SystemTime,
    pub valid_at: Option<SystemTime>,
    pub ip_address: Option<String>,
}

impl ProtectedObject {
    pub fn new(path: String, expires_at: SystemTime) -> Self {
        Self {
            path,
            expires_at,
            ..Default::default()
        }
    }

    pub fn sign(&self) -> Result<String, Error> {
        sign_protected_object(
            &self.path,
            self.expires_at.duration_since(UNIX_EPOCH)?.as_secs(),
            match self.valid_at.as_ref() {
                Some(valid_at) => valid_at.duration_since(UNIX_EPOCH)?.as_secs(),
                None => 0,
            },
            self.ip_address.as_ref().map(String::as_str),
        )
        .map_err(|err| format_err!("{:?}", err))
    }
}

impl Default for ProtectedObject {
    fn default() -> Self {
        Self {
            path: Default::default(),
            expires_at: crate::time::system_time() + std::time::Duration::from_secs(3600),
            valid_at: None,
            ip_address: None,
        }
    }
}
