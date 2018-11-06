use types::StatusCode;

mod raw {
    #[link(wasm_import_module = "/poe/time")]
    extern "C" {
        #[link_name = "/poe/time/monotonic_resolution__1"]
        pub fn monotonic_resolution() -> u64;
        #[link_name = "/poe/time/monotonic_now__1"]
        pub fn monotonic_now(
            precision_index: u32,
            precision: *mut u64,
            timestamp_index: u32,
            timestamp_secs: *mut u64,
            timestamp_sub_index: u32,
            timestamp_subsecs: *mut u32,
        ) -> u32;

        #[link_name = "/poe/time/cpu_resolution__1"]
        pub fn cpu_resolution() -> u64;
        #[link_name = "/poe/time/cpu_now__1"]
        pub fn cpu_now(
            precision_index: u32,
            precision: *mut u64,
            timestamp_index: u32,
            timestamp_secs: *mut u64,
            timestamp_sub_index: u32,
            timestamp_subsecs: *mut u32,
        ) -> u32;

        #[link_name = "/poe/time/os_resolution__1"]
        pub fn os_resolution() -> u64;
        #[link_name = "/poe/time/os_now__1"]
        pub fn os_now(
            precision_index: u32,
            precision: *mut u64,
            timestamp_index: u32,
            timestamp_secs: *mut u64,
            timestamp_sub_index: u32,
            timestamp_subsecs: *mut u32,
        ) -> u32;
    }
}

pub mod monotonic {
    use super::*;

    #[inline]
    pub fn resolution() -> u64 {
        unsafe { raw::monotonic_resolution() }
    }

    pub fn now() -> Result<(u64, u64, u32), StatusCode> {
        let mut precision = 0;
        let mut timestamp_secs = 0;
        let mut timestamp_subsecs = 0;
        let res = unsafe {
            raw::monotonic_now(
                0,
                &mut precision as *mut u64,
                0,
                &mut timestamp_secs as *mut u64,
                0,
                &mut timestamp_subsecs as *mut u32,
            )
        };
        match res.into() {
            StatusCode::SUCCESS => Ok((precision, timestamp_secs, timestamp_subsecs)),
            err => Err(err),
        }
    }
}

pub mod cpu {
    use super::*;

    #[inline]
    pub fn resolution() -> u64 {
        unsafe { raw::cpu_resolution() }
    }

    pub fn now() -> Result<(u64, u64, u32), StatusCode> {
        let mut precision = 0;
        let mut timestamp_secs = 0;
        let mut timestamp_subsecs = 0;
        let res = unsafe {
            raw::cpu_now(
                0,
                &mut precision as *mut u64,
                0,
                &mut timestamp_secs as *mut u64,
                0,
                &mut timestamp_subsecs as *mut u32,
            )
        };
        match res.into() {
            StatusCode::SUCCESS => Ok((precision, timestamp_secs, timestamp_subsecs)),
            err => Err(err),
        }
    }
}

pub mod os {
    use super::*;

    #[inline]
    pub fn resolution() -> u64 {
        unsafe { raw::os_resolution() }
    }

    #[inline]
    pub fn now() -> Result<(u64, u64, u32), StatusCode> {
        let mut precision = 0;
        let mut timestamp_secs = 0;
        let mut timestamp_subsecs = 0;
        let res = unsafe {
            raw::os_now(
                0,
                &mut precision as *mut u64,
                0,
                &mut timestamp_secs as *mut u64,
                0,
                &mut timestamp_subsecs as *mut u32,
            )
        };
        match res.into() {
            StatusCode::SUCCESS => Ok((precision, timestamp_secs, timestamp_subsecs)),
            err => Err(err),
        }
    }
}
