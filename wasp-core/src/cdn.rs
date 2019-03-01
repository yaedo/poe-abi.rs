use crate::types::{read_string, StatusCode};
use alloc::string::String;
use wasp_abi::cdn;

pub fn static_url() -> Result<String, StatusCode> {
    read_string(120, |s, cap| unsafe {
        cdn::read_static_url(0, s as *mut u8, cap as u32)
    })
}

pub fn sign_protected_object(
    path: &str,
    expires_at: u64,
    valid_at: u64,
    ip_address: Option<&str>,
) -> Result<String, StatusCode> {
    let path_ptr = path.as_ptr();
    let path_len = path.len();
    let (ip_ptr, ip_len) = if let Some(ip_address) = ip_address {
        (ip_address.as_ptr(), ip_address.len())
    } else {
        (core::ptr::null(), 0)
    };
    read_string(500, |s, cap| unsafe {
        cdn::sign_protected_object(
            0,
            path_ptr,
            path_len as u32,
            expires_at,
            valid_at,
            0,
            ip_ptr,
            ip_len as u32,
            0,
            s as *mut u8,
            cap as u32,
        )
    })
}
