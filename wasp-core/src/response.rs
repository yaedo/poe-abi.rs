use crate::types::{AbiResult, StatusCode};
use wasp_abi::response;

#[inline]
pub fn write_status_code(code: u16) -> Result<(), StatusCode> {
    let code: StatusCode = unsafe { response::write_status_code(code as u32) }.into();
    code.into()
}

pub fn write_header(name: &str, value: &str) -> Result<usize, StatusCode> {
    let code: AbiResult = unsafe {
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
    let res: Result<_, _> = code.into();
    res.map(|len| len as usize)
}

pub fn end_head() -> Result<(), StatusCode> {
    let res: StatusCode = unsafe { response::end_head() }.into();
    res.into()
}

pub fn write_body(data: &[u8]) -> Result<usize, StatusCode> {
    let res: AbiResult =
        unsafe { response::write_body(0, data.as_ptr(), data.len() as u32) }.into();
    let res: Result<_, _> = res.into();
    res.map(|len| len as usize)
}

pub fn end_body() -> Result<(), StatusCode> {
    let res: StatusCode = unsafe { response::end_body() }.into();
    res.into()
}
