use alloc::string::String;
use core::str;
use types::read_string;
use types::StatusCode;

mod raw {
    use types::Result;
    #[link(wasm_import_module = "/poe/env")]
    extern "system" {
        #[link_name = "/poe/env/len__1"]
        pub fn len() -> u32;

        #[link_name = "/poe/env/key_at_index__1"]
        pub fn key_at_index(index: u32, mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> Result;

        #[link_name = "/poe/env/value_at_index__1"]
        pub fn value_at_index(
            index: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> Result;

        #[link_name = "/poe/env/get__1"]
        pub fn get(
            key_index: u32,
            key_addr: *const u8,
            key_len: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> Result;
    }
}

#[inline]
pub fn len() -> u32 {
    unsafe { raw::len() }
}

pub fn key_at_index(index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        raw::key_at_index(index, 0, s as *mut u8, cap as u32)
    })
}

pub fn value_at_index(index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        raw::value_at_index(index, 0, s as *mut u8, cap as u32)
    })
}

pub fn get(key: &str) -> Result<String, StatusCode> {
    let ptr = key.as_ptr();
    let len = key.len() as u32;
    read_string(32, |s, cap| unsafe {
        raw::get(0, ptr, len, 0, s as *mut u8, cap as u32)
    })
}

pub fn iter() -> EnvIterator {
    let len = len();
    EnvIterator { len, i: 0 }
}

pub struct EnvIterator {
    i: u32,
    len: u32,
}

impl Iterator for EnvIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.i;
        if index >= self.len {
            None
        } else {
            let key = key_at_index(index).ok()?;
            let value = value_at_index(index).ok()?;
            self.i += 1;
            Some((key, value))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len as usize;
        (len, Some(len))
    }
}
