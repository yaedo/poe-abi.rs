use alloc::string::String;
use crate::types::{read_string, AbiResult, StatusCode};
use wasp_abi::http;

#[derive(Debug)]
pub struct Request(u32);

impl Drop for Request {
    fn drop(&mut self) {
        unsafe { http::close(self.0) };
    }
}

impl Request {
    pub fn open(method: &str, uri: &str) -> Result<Request, StatusCode> {
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

        res.map(Request)
    }

    pub fn write_header(&mut self, name: &str, value: &str) -> Result<(), StatusCode> {
        let res: StatusCode = unsafe {
            http::write_header(
                self.0,
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

    pub fn send_head(&mut self) -> Result<(), StatusCode> {
        let res: StatusCode = unsafe { http::send_head(self.0) }.into();

        res.into()
    }

    pub fn write_body(&mut self, data: &[u8]) -> Result<usize, StatusCode> {
        let res: AbiResult =
            unsafe { http::write_body(self.0, 0, data.as_ptr(), data.len() as u32) }.into();

        let res: Result<_, _> = res.into();
        res.map(|v| v as usize)
    }

    pub fn send_body(&mut self) -> Result<(), StatusCode> {
        let res: StatusCode = unsafe { http::send_body(self.0) }.into();
        res.into()
    }

    pub fn read_status_code(&self) -> Result<usize, StatusCode> {
        let res: AbiResult = unsafe { http::read_status_code(self.0) }.into();
        let res: Result<_, _> = res.into();
        res.map(|v| v as usize)
    }

    pub fn read_header_len(&self) -> Result<usize, StatusCode> {
        let res: AbiResult = unsafe { http::read_header_len(self.0) }.into();
        let res: Result<_, _> = res.into();
        res.map(|v| v as usize)
    }

    pub fn read_body(&self, out: &mut [u8]) -> Result<usize, StatusCode> {
        let res: AbiResult =
            unsafe { http::read_body(self.0, 0, out.as_mut_ptr(), out.len() as u32) }.into();
        let res: Result<_, _> = res.into();
        res.map(|v| v as usize)
    }

    pub fn read_header_value(&self, name: &str) -> Result<String, StatusCode> {
        let ptr = name.as_ptr();
        let len = name.len() as u32;
        read_string(32, |s, cap| unsafe {
            http::read_header_value(self.0, 0, ptr, len, 0, s as *mut u8, cap as u32)
        })
    }

    pub fn read_headers(&self) -> Result<HeaderIterator, StatusCode> {
        let len = self.read_header_len()?;
        Ok(HeaderIterator {
            len,
            i: 0,
            request: self,
        })
    }

    pub fn read_header_name_at_index(&self, index: usize) -> Result<String, StatusCode> {
        read_string(32, |s, cap| unsafe {
            http::read_header_name_at_index(self.0, index as u32, 0, s as *mut u8, cap as u32)
        })
    }

    pub fn read_header_value_at_index(&self, index: usize) -> Result<String, StatusCode> {
        read_string(32, |s, cap| unsafe {
            http::read_header_value_at_index(self.0, index as u32, 0, s as *mut u8, cap as u32)
        })
    }
}

pub struct HeaderIterator<'a> {
    i: usize,
    request: &'a Request,
    len: usize,
}

impl<'a> Iterator for HeaderIterator<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let request = self.request;
        let index = self.i;
        if index >= self.len {
            None
        } else {
            let name = request.read_header_name_at_index(index).ok()?;
            let value = request.read_header_value_at_index(index).ok()?;
            self.i += 1;
            Some((name, value))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len as usize;
        (len, Some(len))
    }
}
