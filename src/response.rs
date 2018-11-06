use types::{AbiResult, StatusCode};

mod raw {
    #[link(wasm_import_module = "/poe/response")]
    extern "C" {
        #[link_name = "/poe/response/write_status_code__1"]
        pub fn write_status_code(code: u32) -> u32;

        #[link_name = "/poe/response/write_header__1"]
        pub fn write_header(
            name_index: u32,
            name_addr: *const u8,
            name_len: u32,
            value_index: u32,
            value_addr: *const u8,
            value_len: u32,
        ) -> u32;

        #[link_name = "/poe/response/end_head__1"]
        pub fn end_head() -> u64;

        #[link_name = "/poe/response/write_body__1"]
        pub fn write_body(data_index: u32, data_addr: *const u8, data_len: u32) -> u64;

        #[link_name = "/poe/response/end_body__1"]
        pub fn end_body() -> u64;
    }
}

#[inline]
pub fn write_status_code(code: u32) -> Result<(), StatusCode> {
    let code: StatusCode = unsafe { raw::write_status_code(code) }.into();
    code.into()
}

pub fn write_header(name: &str, value: &str) -> Result<(), StatusCode> {
    let code: StatusCode = unsafe {
        raw::write_header(
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
    let res: AbiResult = unsafe { raw::end_head() }.into();
    res.into()
}

pub fn write_body(data: &[u8]) -> Result<u32, StatusCode> {
    let res: AbiResult = unsafe { raw::write_body(0, data.as_ptr(), data.len() as u32) }.into();
    res.into()
}

pub fn end_body() -> Result<u32, StatusCode> {
    let res: AbiResult = unsafe { raw::end_body() }.into();
    res.into()
}
