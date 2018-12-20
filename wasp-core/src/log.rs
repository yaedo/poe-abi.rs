use crate::types::{AbiResult, StatusCode};
use wasp_abi::log;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Level {
    Emergency = 0,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Informational,
    Debug,
}

impl Default for Level {
    fn default() -> Self {
        Level::Informational
    }
}

#[inline]
pub fn write<Data: AsRef<str>>(level: Level, data: Data) -> Result<usize, StatusCode> {
    let data = data.as_ref();
    let ptr = data.as_ptr();
    let res: AbiResult = unsafe { log::write(level as u32, 0, ptr, data.len() as u32) }.into();
    let res: Result<_, _> = res.into();
    res.map(|len| len as usize)
}
