use alloc::string::String;
use types::read_string;
use types::{AbiResult, StatusCode};

mod raw {
    use types::Result;
    #[link(wasm_import_module = "/poe/request")]
    extern "C" {
        #[link_name = "/poe/request/read_method__1"]
        pub fn read_method(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> Result;

        #[link_name = "/poe/request/read_uri__1"]
        pub fn read_uri(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> Result;

        #[link_name = "/poe/request/read_path__1"]
        pub fn read_path(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> Result;

        #[link_name = "/poe/request/read_query__1"]
        pub fn read_query(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> Result;

        #[link_name = "/poe/request/read_header_len__1"]
        pub fn read_header_len() -> u32;

        #[link_name = "/poe/request/read_header_name_at_index__1"]
        pub fn read_header_name_at_index(
            index: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> Result;

        #[link_name = "/poe/request/read_header_value_at_index__1"]
        pub fn read_header_value_at_index(
            index: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> Result;

        #[link_name = "/poe/request/read_header_value__1"]
        pub fn read_header_value(
            key_index: u32,
            key_addr: *const u8,
            key_len: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> Result;

        #[link_name = "/poe/request/read_body__1"]
        pub fn read_body(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> Result;
    }
}

pub fn read_method() -> Result<String, StatusCode> {
    read_string(8, |s, cap| unsafe {
        raw::read_method(0, s as *mut u8, cap as u32)
    })
}

pub fn read_uri() -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        raw::read_uri(0, s as *mut u8, cap as u32)
    })
}

pub fn read_path() -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        raw::read_path(0, s as *mut u8, cap as u32)
    })
}

pub fn read_query() -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        raw::read_query(0, s as *mut u8, cap as u32)
    })
}

#[inline]
pub fn read_header_len() -> u32 {
    unsafe { raw::read_header_len() }
}

pub fn read_header_name_at_index(index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        raw::read_header_name_at_index(index, 0, s as *mut u8, cap as u32)
    })
}

pub fn read_header_value_at_index(index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        raw::read_header_value_at_index(index, 0, s as *mut u8, cap as u32)
    })
}

pub fn read_header_value(name: &str) -> Result<String, StatusCode> {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    read_string(32, |s, cap| unsafe {
        raw::read_header_value(0, ptr, len, 0, s as *mut u8, cap as u32)
    })
}

pub fn read_headers() -> HeaderIterator {
    let len = read_header_len();
    HeaderIterator { len, i: 0 }
}

pub struct HeaderIterator {
    i: u32,
    len: u32,
}

impl Iterator for HeaderIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.i;
        if index >= self.len {
            None
        } else {
            let name = read_header_name_at_index(index).ok()?;
            let value = read_header_value_at_index(index).ok()?;
            self.i += 1;
            Some((name, value))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len as usize;
        (len, Some(len))
    }
}

pub fn read_body(out: &mut [u8]) -> Option<usize> {
    match unsafe { raw::read_body(0, out.as_mut_ptr(), out.len() as u32) }.into() {
        AbiResult(StatusCode::SUCCESS, len) => Some(len as usize),
        _ => None,
    }
}
