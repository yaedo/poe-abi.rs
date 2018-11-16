use crate::types::StatusCode;
use wasp_abi::time;

pub mod monotonic {
    use super::*;

    #[inline]
    pub fn resolution() -> u64 {
        unsafe { time::monotonic_resolution() }
    }

    pub fn now() -> Result<(u64, u64, u32), StatusCode> {
        let mut precision = 0;
        let mut timestamp_secs = 0;
        let mut timestamp_subsecs = 0;
        let res = unsafe {
            time::monotonic_now(
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
        unsafe { time::cpu_resolution() }
    }

    pub fn now() -> Result<(u64, u64, u32), StatusCode> {
        let mut precision = 0;
        let mut timestamp_secs = 0;
        let mut timestamp_subsecs = 0;
        let res = unsafe {
            time::cpu_now(
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
        unsafe { time::os_resolution() }
    }

    #[inline]
    pub fn now() -> Result<(u64, u64, u32), StatusCode> {
        let mut precision = 0;
        let mut timestamp_secs = 0;
        let mut timestamp_subsecs = 0;
        let res = unsafe {
            time::os_now(
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
