use wasp_abi::process;

#[inline]
pub fn yield_() {
    unsafe { process::yield_() }
}

#[inline]
pub fn nanosleep(duration: u64) {
    unsafe { process::nanosleep(duration) }
}

#[inline]
pub fn getpid() -> u32 {
    unsafe { process::getpid() }
}

#[inline]
pub fn getppid() -> u32 {
    unsafe { process::getppid() }
}

#[inline]
pub fn exit(rval_: u32) -> ! {
    unsafe { process::exit(rval_) }
}
