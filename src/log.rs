use types::StatusCode;

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

mod raw {
    #[link(wasm_import_module = "/poe/log")]
    extern "C" {
        #[link_name = "/poe/log/write__1"]
        pub fn write(level: u32, memory_index: u32, memory_addr: *const u8, memory_len: u32)
            -> u32;
    }
}

#[inline]
pub fn write(level: Level, data: &str) -> Result<(), StatusCode> {
    let ptr = data.as_ptr();
    let code: StatusCode = unsafe { raw::write(level as u32, 0, ptr, data.len() as u32) }.into();
    code.into()
}
