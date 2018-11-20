use alloc::string::String;
use core::str;
use crate::types::{read_string, StatusCode};
use wasp_abi::env;

#[inline]
pub fn len() -> u32 {
    unsafe { env::len() }
}

pub fn key_at_index(index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        env::key_at_index(index, 0, s as *mut u8, cap as u32)
    })
}

pub fn value_at_index(index: u32) -> Result<String, StatusCode> {
    read_string(32, |s, cap| unsafe {
        env::value_at_index(index, 0, s as *mut u8, cap as u32)
    })
}

pub fn get(key: &str) -> Result<String, StatusCode> {
    let ptr = key.as_ptr();
    let len = key.len() as u32;
    read_string(32, |s, cap| unsafe {
        env::get(0, ptr, len, 0, s as *mut u8, cap as u32)
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
