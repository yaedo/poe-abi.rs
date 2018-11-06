use alloc::string::String;
use core::mem::replace;
use core::u32::MAX as MAX_U32;
use types::read_string;
use types::{AbiResult, StatusCode};

mod raw {
    use types::Result;
    #[link(wasm_import_module = "/poe/http")]
    extern "C" {
        #[link_name = "/poe/http/open__1"]
        pub fn open(
            method_mem_index: u32,
            method_mem_addr: *const u8,
            method_mem_len: u32,
            url_mem_index: u32,
            url_mem_addr: *const u8,
            url_mem_len: u32,
        ) -> Result;

        #[link_name = "/poe/http/write_header__1"]
        pub fn write_header(
            id: u32,
            name_index: u32,
            name_addr: *const u8,
            name_len: u32,
            value_index: u32,
            value_addr: *const u8,
            value_len: u32,
        ) -> u32;

        #[link_name = "/poe/http/write_body__1"]
        pub fn write_body(id: u32, mem_index: u32, mem_addr: *const u8, mem_len: u32) -> Result;

        #[link_name = "/poe/http/send__1"]
        pub fn send(id: u32) -> u32;

        #[link_name = "/poe/http/read_status_code__1"]
        pub fn read_status_code(id: u32) -> Result;

        #[link_name = "/poe/http/read_header_len__1"]
        pub fn read_header_len(id: u32) -> Result;

        #[link_name = "/poe/http/read_header_name_at_index__1"]
        pub fn read_header_name_at_index(
            id: u32,
            index: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> Result;

        #[link_name = "/poe/http/read_header_value_at_index__1"]
        pub fn read_header_value_at_index(
            id: u32,
            index: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> Result;

        #[link_name = "/poe/http/read_header_value__1"]
        pub fn read_header_value(
            id: u32,
            key_index: u32,
            key_addr: *const u8,
            key_len: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> Result;

        #[link_name = "/poe/http/read_body__1"]
        pub fn read_body(id: u32, mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> Result;

        #[link_name = "/poe/http/close__1"]
        pub fn close(id: u32) -> u32;
    }
}

#[derive(Clone, Debug)]
pub struct HttpRequest {
    id: u32,
}

impl HttpRequest {
    pub fn open(method: &str, uri: &str) -> Result<Self, StatusCode> {
        let res: AbiResult = unsafe {
            raw::open(
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
            raw::write_header(
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
            unsafe { raw::write_body(self.id, 0, data.as_ptr(), data.len() as u32) }.into();

        res.into()
    }

    pub fn send(mut self) -> Result<HttpResponse, StatusCode> {
        let id = replace(&mut self.id, MAX_U32);
        let res: StatusCode = unsafe { raw::send(id) }.into();
        let res: Result<(), StatusCode> = res.into();

        res.map(|_| HttpResponse { id })
    }
}

impl Drop for HttpRequest {
    fn drop(&mut self) {
        if self.id != MAX_U32 {
            unsafe { raw::close(self.id) };
        }
    }
}

#[derive(Clone, Debug)]
pub struct HttpResponse {
    id: u32,
}

impl HttpResponse {
    pub fn read_status_code(&self) -> Result<u32, StatusCode> {
        let res: AbiResult = unsafe { raw::read_status_code(self.id) }.into();
        res.into()
    }

    pub fn read_header_len(&self) -> Result<u32, StatusCode> {
        let res: AbiResult = unsafe { raw::read_header_len(self.id) }.into();
        res.into()
    }

    pub fn read_body(&self, out: &mut [u8]) -> Result<u32, StatusCode> {
        let res: AbiResult =
            unsafe { raw::read_body(self.id, 0, out.as_mut_ptr(), out.len() as u32) }.into();
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
            raw::read_header_value(self.id, 0, ptr, len, 0, s as *mut u8, cap as u32)
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
        raw::read_header_name_at_index(id, index, 0, s as *mut u8, cap as u32)
    })
}

fn read_header_value_at_index(id: u32, index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        raw::read_header_value_at_index(id, index, 0, s as *mut u8, cap as u32)
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
        unsafe { raw::close(self.id) };
    }
}
