use alloc::{string::String, vec::Vec};
use core::mem::{forget, size_of};
pub use wasp_status_code::StatusCode;

pub type Result = u64;
pub struct AbiResult(pub StatusCode, pub u32);

impl From<Result> for AbiResult {
    fn from(value: Result) -> Self {
        let param = (value >> (size_of::<u32>() * 8)) as u32;
        let code = (value as u32).into();
        AbiResult(code, param)
    }
}

impl Into<::core::result::Result<u32, StatusCode>> for AbiResult {
    fn into(self) -> ::core::result::Result<u32, StatusCode> {
        match self {
            AbiResult(StatusCode::Success, value) => Ok(value),
            AbiResult(code, _) => Err(code),
        }
    }
}

macro_rules! reader {
    ($mod:ident, $ptr:ident, $capacity:expr, $read:expr) => {{
        let mut value = $mod::with_capacity($capacity);
        let len;

        loop {
            match $read(value.$ptr(), value.capacity()).into() {
                AbiResult(StatusCode::Success, l) => {
                    len = l as usize;
                    break;
                }
                AbiResult(StatusCode::Range, new_size) => {
                    value.reserve_exact(new_size as usize);
                }
                AbiResult(status, _) => return Err(status),
            }
        }

        let ptr = value.as_ptr();
        let capacity = value.capacity();
        forget(value);

        let mut value = unsafe { $mod::from_raw_parts(ptr as *mut _, len, capacity) };
        value.shrink_to_fit();
        Ok(value)
    }};
}

#[inline]
pub fn read_vec<F: (Fn(*mut &[u8], usize) -> u64)>(
    capacity: usize,
    read: F,
) -> core::result::Result<Vec<u8>, StatusCode> {
    reader!(Vec, as_mut_ptr, capacity, read)
}

#[inline]
pub fn read_string<F: (Fn(*mut str, usize) -> u64)>(
    capacity: usize,
    read: F,
) -> core::result::Result<String, StatusCode> {
    reader!(String, as_mut_str, capacity, read)
}
