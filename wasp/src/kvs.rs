use std::io::{Error as IOError, ErrorKind as IOErrorKind, Read, Write};
use wasp_core::{kvs, types::StatusCode};

#[derive(Debug)]
pub struct Get(kvs::Get);

pub fn get(key: &str) -> Result<Get, IOError> {
    Get::open(key)
}

impl Get {
    pub fn open(key: &str) -> Result<Self, IOError> {
        kvs::get(key).map(Get).map_err(map_status_code)
    }
}

impl Read for Get {
    fn read(&mut self, out: &mut [u8]) -> Result<usize, IOError> {
        self.0.read(out).map_err(map_status_code)
    }
}

#[derive(Debug)]
pub struct List(kvs::List);

pub fn list(prefix: &str) -> Result<List, IOError> {
    List::open(prefix)
}

impl List {
    pub fn open(key: &str) -> Result<Self, IOError> {
        kvs::list(key).map(List).map_err(map_status_code)
    }
}

impl Iterator for List {
    type Item = Result<String, IOError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|v| v.map_err(map_status_code))
    }
}

#[derive(Debug)]
pub struct Put(kvs::Put);

pub fn put(key: &str, content_length: u32, cache_control: u32) -> Result<Put, IOError> {
    Put::open(key, content_length, cache_control)
}

pub fn put_new(key: &str, content_length: u32, cache_control: u32) -> Result<Put, IOError> {
    Put::open_new(key, content_length, cache_control)
}

impl Put {
    pub fn open_new(key: &str, content_length: u32, cache_control: u32) -> Result<Self, IOError> {
        kvs::put_new(key, content_length, cache_control)
            .map(Put)
            .map_err(map_status_code)
    }

    pub fn open(key: &str, content_length: u32, cache_control: u32) -> Result<Self, IOError> {
        kvs::put(key, content_length, cache_control)
            .map(Put)
            .map_err(map_status_code)
    }
}

impl Write for Put {
    fn flush(&mut self) -> Result<(), IOError> {
        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, IOError> {
        self.0.write(data).map_err(map_status_code)
    }
}

pub fn delete(key: &str) -> Result<(), IOError> {
    kvs::delete(key).map_err(map_status_code)
}

pub fn copy(from: &str, to: &str) -> Result<(), IOError> {
    kvs::copy(from, to).map_err(map_status_code)
}

pub fn rename(from: &str, to: &str) -> Result<(), IOError> {
    kvs::rename(from, to).map_err(map_status_code)
}

fn map_status_code(err: StatusCode) -> IOError {
    // TODO better mapping
    IOError::new(IOErrorKind::Other, format!("{:?}", err))
}
