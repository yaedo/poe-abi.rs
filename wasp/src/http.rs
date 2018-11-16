use alloc::string::String;
use core::{mem::replace, u32::MAX as MAX_U32};
use crate::types::{read_string, AbiResult, StatusCode};
use wasp_abi::http;

#[derive(Debug)]
pub struct HttpRequest {
    id: u32,
}

impl HttpRequest {
    pub fn open(method: &str, uri: &str) -> Result<Self, StatusCode> {
        let res: AbiResult = unsafe {
            http::open(
                0,
                method.as_ptr(),
                method.len() as u32,
                0,
                uri.as_ptr(),
                uri.len() as u32,
            )
        }
        .into();
        let res: Result<u32, StatusCode> = res.into();

        res.map(|id| Self { id })
    }

    pub fn write_header(&mut self, name: &str, value: &str) -> Result<(), StatusCode> {
        let res: StatusCode = unsafe {
            http::write_header(
                self.id,
                0,
                name.as_ptr(),
                name.len() as u32,
                0,
                value.as_ptr(),
                value.len() as u32,
            )
        }
        .into();

        res.into()
    }

    pub fn write_body(&mut self, data: &[u8]) -> Result<u32, StatusCode> {
        let res: AbiResult =
            unsafe { http::write_body(self.id, 0, data.as_ptr(), data.len() as u32) }.into();

        res.into()
    }

    pub fn send(mut self) -> Result<HttpResponse, StatusCode> {
        let id = replace(&mut self.id, MAX_U32);
        let res: StatusCode = unsafe { http::send(id) }.into();
        let res: Result<(), StatusCode> = res.into();

        res.map(|_| HttpResponse { id })
    }
}

impl Drop for HttpRequest {
    fn drop(&mut self) {
        if self.id != MAX_U32 {
            unsafe { http::close(self.id) };
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    id: u32,
}

impl HttpResponse {
    pub fn read_status_code(&self) -> Result<u32, StatusCode> {
        let res: AbiResult = unsafe { http::read_status_code(self.id) }.into();
        res.into()
    }

    pub fn read_header_len(&self) -> Result<u32, StatusCode> {
        let res: AbiResult = unsafe { http::read_header_len(self.id) }.into();
        res.into()
    }

    pub fn read_body(&self, out: &mut [u8]) -> Result<u32, StatusCode> {
        let res: AbiResult =
            unsafe { http::read_body(self.id, 0, out.as_mut_ptr(), out.len() as u32) }.into();
        res.into()
    }

    pub fn read_header_name_at_index(&self, index: u32) -> Result<String, StatusCode> {
        read_header_name_at_index(self.id, index)
    }

    pub fn read_header_value_at_index(&self, index: u32) -> Result<String, StatusCode> {
        read_header_value_at_index(self.id, index)
    }

    pub fn read_header_value(&self, name: &str) -> Result<String, StatusCode> {
        let ptr = name.as_ptr();
        let len = name.len() as u32;
        read_string(32, |s, cap| unsafe {
            http::read_header_value(self.id, 0, ptr, len, 0, s as *mut u8, cap as u32)
        })
    }

    pub fn read_headers(&self) -> Result<HeaderIterator, StatusCode> {
        let id = self.id;
        let len = self.read_header_len()?;
        Ok(HeaderIterator { len, i: 0, id })
    }
}

fn read_header_name_at_index(id: u32, index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        http::read_header_name_at_index(id, index, 0, s as *mut u8, cap as u32)
    })
}

fn read_header_value_at_index(id: u32, index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        http::read_header_value_at_index(id, index, 0, s as *mut u8, cap as u32)
    })
}

pub struct HeaderIterator {
    i: u32,
    id: u32,
    len: u32,
}

impl Iterator for HeaderIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.id;
        let index = self.i;
        if index >= self.len {
            None
        } else {
            let name = read_header_name_at_index(id, index).ok()?;
            let value = read_header_value_at_index(id, index).ok()?;
            self.i += 1;
            Some((name, value))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len as usize;
        (len, Some(len))
    }
}

impl Drop for HttpResponse {
    fn drop(&mut self) {
        unsafe { http::close(self.id) };
    }
}
