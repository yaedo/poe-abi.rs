use crate::types::StatusCode;
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
pub fn write(level: Level, data: &str) -> Result<(), StatusCode> {
    let ptr = data.as_ptr();
    let code: StatusCode = unsafe { log::write(level as u32, 0, ptr, data.len() as u32) }.into();
    code.into()
}
