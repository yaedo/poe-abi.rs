use alloc::vec::Vec;
use types::read_vec;
use types::StatusCode;

mod raw {
    use types::Result;
    #[link(wasm_import_module = "/poe/args")]
    extern "system" {
        #[link_name = "/poe/args/len__1"]
        pub fn len() -> u32;

        #[link_name = "/poe/args/get__1"]
        pub fn get(index: u32, mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> Result;
    }
}

#[inline]
pub fn len() -> u32 {
    unsafe { raw::len() }
}

pub fn get(index: u32) -> Result<Vec<u8>, StatusCode> {
    read_vec(32, |s, cap| unsafe {
        raw::get(index, 0, s as *mut u8, cap as u32)
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
