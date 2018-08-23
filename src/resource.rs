#[link(wasm_import_module = "poe")]
extern "C" {
    pub fn resource_open(url_ptr: *const u8, url_len: usize) -> i32;
    pub fn resource_read(id: i32, data_ptr: *mut u8, data_len: usize) -> i32;
    pub fn resource_write(id: i32, data_ptr: *const u8, data_len: usize) -> i32;
    pub fn resource_meta(id: i32, key_ptr: *const u8, key_len: usize) -> i32;
    pub fn resource_flush(id: i32) -> i32;
    pub fn resource_close(id: i32);
}

use std::io;
use std::io::{Read, Write};

pub struct Resource {
    handle: i32,
}

impl Resource {
    pub fn open(url: &[u8]) -> Option<Self> {
        if url.len() == 0 {
            return None;
        }

        let handle = unsafe { resource_open(&url[0], url.len()) };
        if handle < 0 {
            None
        } else {
            Some(Resource { handle: handle })
        }
    }

    pub fn read(&self, out: &mut [u8]) -> io::Result<usize> {
        let len = out.len();

        if len == 0 {
            return Ok(0);
        }

        let ret = unsafe { resource_read(self.handle, &mut out[0], len) };

        if ret < 0 {
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            Ok(ret as usize)
        }
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        let len = data.len();

        if len == 0 {
            return Ok(0);
        }

        let ret = unsafe { resource_write(self.handle, &data[0], len) };
        if ret < 0 {
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            Ok(ret as usize)
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        let ret = unsafe { resource_flush(self.handle) };
        if ret < 0 {
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            Ok(())
        }
    }

    pub fn meta(&self, key: &[u8]) -> Option<Self> {
        if key.len() == 0 {
            return None;
        }

        let handle = unsafe { resource_meta(self.handle, &key[0], key.len()) };
        if handle < 0 {
            None
        } else {
            Some(Resource { handle: handle })
        }
    }

    pub unsafe fn from_raw(handle: i32) -> Option<Resource> {
        if handle > 0 {
            Some(Resource { handle })
        } else {
            None
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            resource_close(self.handle);
        }
    }
}

impl Read for Resource {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        Resource::read(self, out)
    }
}

impl Write for Resource {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        Self::write(self, data)
    }

    fn flush(&mut self) -> io::Result<()> {
        Self::flush(self)
    }
}
