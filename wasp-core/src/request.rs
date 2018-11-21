use alloc::string::String;
use crate::types::{read_string, AbiResult, StatusCode};
use wasp_abi::request;

pub fn read_method() -> Result<String, StatusCode> {
    read_string(8, |s, cap| unsafe {
        request::read_method(0, s as *mut u8, cap as u32)
    })
}

pub fn read_uri() -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        request::read_uri(0, s as *mut u8, cap as u32)
    })
}

pub fn read_path() -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        request::read_path(0, s as *mut u8, cap as u32)
    })
}

pub fn read_query() -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        request::read_query(0, s as *mut u8, cap as u32)
    })
}

#[inline]
pub fn read_header_len() -> u32 {
    unsafe { request::read_header_len() }
}

pub fn read_header_name_at_index(index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        request::read_header_name_at_index(index, 0, s as *mut u8, cap as u32)
    })
}

pub fn read_header_value_at_index(index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        request::read_header_value_at_index(index, 0, s as *mut u8, cap as u32)
    })
}

pub fn read_header_value(name: &str) -> Result<String, StatusCode> {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    read_string(32, |s, cap| unsafe {
        request::read_header_value(0, ptr, len, 0, s as *mut u8, cap as u32)
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

pub fn read_body(out: &mut [u8]) -> Result<usize, StatusCode> {
    let res: AbiResult =
        unsafe { request::read_body(0, out.as_mut_ptr(), out.len() as u32) }.into();
    let res: Result<_, _> = res.into();
    res.map(|len| len as usize)
}
