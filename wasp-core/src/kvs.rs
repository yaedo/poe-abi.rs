use crate::types::{read_string, AbiResult, StatusCode};
use alloc::string::String;
use wasp_abi::kvs;

#[derive(Debug)]
pub struct Get(u32);

impl Drop for Get {
    fn drop(&mut self) {
        unsafe { kvs::get_close(self.0) };
    }
}

pub fn get(key: &str) -> Result<Get, StatusCode> {
    Get::open(key)
}

impl Get {
    pub fn open(key: &str) -> Result<Self, StatusCode> {
        let res: AbiResult = unsafe { kvs::get_open(0, key.as_ptr(), key.len() as u32) }.into();
        let res: Result<u32, StatusCode> = res.into();

        res.map(Get)
    }

    pub fn read(&self, out: &mut [u8]) -> Result<usize, StatusCode> {
        let res: AbiResult =
            unsafe { kvs::get_read(self.0, 0, out.as_mut_ptr(), out.len() as u32) }.into();
        let res: Result<_, _> = res.into();
        res.map(|v| v as usize)
    }
}

#[derive(Debug)]
pub struct List(u32);

pub fn list(prefix: &str) -> Result<List, StatusCode> {
    List::open(prefix)
}

impl Drop for List {
    fn drop(&mut self) {
        unsafe {
            kvs::list_close(self.0);
        }
    }
}

impl List {
    pub fn open(prefix: &str) -> Result<Self, StatusCode> {
        let res: AbiResult =
            unsafe { kvs::list_open(0, prefix.as_ptr(), prefix.len() as u32) }.into();
        let res: Result<u32, StatusCode> = res.into();

        res.map(List)
    }

    pub fn read(&self) -> Result<String, StatusCode> {
        let id = self.0;
        read_string(32, |s, cap| unsafe {
            kvs::list_read(id, 0, s as *mut u8, cap as u32)
        })
    }
}

impl Iterator for List {
    type Item = Result<String, StatusCode>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read() {
            Ok(item) => Some(Ok(item)),
            Err(StatusCode::UnexpectedEof) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

#[derive(Debug)]
pub struct Put(u32);

pub fn put(key: &str, content_length: u32, cache_control: u32) -> Result<Put, StatusCode> {
    Put::open(key, content_length, cache_control)
}

pub fn put_new(key: &str, content_length: u32, cache_control: u32) -> Result<Put, StatusCode> {
    Put::open_new(key, content_length, cache_control)
}

impl Drop for Put {
    fn drop(&mut self) {
        unsafe {
            kvs::put_close(self.0);
        }
    }
}

impl Put {
    pub fn open(key: &str, content_length: u32, cache_control: u32) -> Result<Self, StatusCode> {
        let res: AbiResult = unsafe {
            kvs::put_open(
                0,
                key.as_ptr(),
                key.len() as u32,
                content_length,
                cache_control,
            )
        }
        .into();
        let res: Result<u32, StatusCode> = res.into();

        res.map(Put)
    }

    pub fn open_new(
        key: &str,
        content_length: u32,
        cache_control: u32,
    ) -> Result<Self, StatusCode> {
        let res: AbiResult = unsafe {
            kvs::put_new_open(
                0,
                key.as_ptr(),
                key.len() as u32,
                content_length,
                cache_control,
            )
        }
        .into();
        let res: Result<u32, StatusCode> = res.into();

        res.map(Put)
    }

    pub fn write(&self, out: &[u8]) -> Result<usize, StatusCode> {
        let res: AbiResult =
            unsafe { kvs::put_write(self.0, 0, out.as_ptr(), out.len() as u32) }.into();
        let res: Result<_, _> = res.into();
        res.map(|v| v as usize)
    }
}

pub fn delete(key: &str) -> Result<(), StatusCode> {
    let res: StatusCode = unsafe { kvs::delete(0, key.as_ptr(), key.len() as u32) }.into();
    res.into()
}

pub fn copy(from: &str, to: &str) -> Result<(), StatusCode> {
    let res: StatusCode = unsafe {
        kvs::copy(
            0,
            from.as_ptr(),
            from.len() as u32,
            0,
            to.as_ptr(),
            to.len() as u32,
        )
    }
    .into();
    res.into()
}

pub fn rename(from: &str, to: &str) -> Result<(), StatusCode> {
    let res: StatusCode = unsafe {
        kvs::rename(
            0,
            from.as_ptr(),
            from.len() as u32,
            0,
            to.as_ptr(),
            to.len() as u32,
        )
    }
    .into();
    res.into()
}
