use crate::types::{AbiResult, StatusCode};
use wasp_abi::response;

#[inline]
pub fn write_status_code(code: u32) -> Result<(), StatusCode> {
    let code: StatusCode = unsafe { response::write_status_code(code) }.into();
    code.into()
}

pub fn write_header(name: &str, value: &str) -> Result<(), StatusCode> {
    let code: StatusCode = unsafe {
        response::write_header(
            0,
            name.as_ptr(),
            name.len() as u32,
            0,
            value.as_ptr(),
            value.len() as u32,
        )
    }
    .into();
    code.into()
}

pub fn end_head() -> Result<u32, StatusCode> {
    let res: AbiResult = unsafe { response::end_head() }.into();
    res.into()
}

pub fn write_body(data: &[u8]) -> Result<u32, StatusCode> {
    let res: AbiResult =
        unsafe { response::write_body(0, data.as_ptr(), data.len() as u32) }.into();
    res.into()
}

pub fn end_body() -> Result<u32, StatusCode> {
    let res: AbiResult = unsafe { response::end_body() }.into();
    res.into()
}
