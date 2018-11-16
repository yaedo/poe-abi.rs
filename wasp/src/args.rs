use alloc::vec::Vec;
use crate::types::read_vec;
use crate::types::StatusCode;
use wasp_abi::args;

#[inline]
pub fn len() -> u32 {
    unsafe { args::len() }
}

pub fn get(index: u32) -> Result<Vec<u8>, StatusCode> {
    read_vec(32, |s, cap| unsafe {
        args::get(index, 0, s as *mut u8, cap as u32)
    })
}

pub fn iter() -> ArgsIterator {
    let len = len();
    ArgsIterator { len, i: 0 }
}

pub struct ArgsIterator {
    i: u32,
    len: u32,
}

impl Iterator for ArgsIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.i;
        if index >= self.len {
            None
        } else {
            let res = get(index).ok();
            self.i += 1;
            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len as usize;
        (len, Some(len))
    }
}
