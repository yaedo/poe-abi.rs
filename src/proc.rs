mod raw {
    #[link(wasm_import_module = "/poe/proc")]
    extern "C" {
        #[link_name = "/poe/proc/yield__1"]
        pub fn yield_();
        #[link_name = "/poe/proc/nanosleep__1"]
        pub fn nanosleep(duration: u64);
        #[link_name = "/poe/proc/getpid__1"]
        pub fn getpid() -> u32;
        #[link_name = "/poe/proc/getppid__1"]
        pub fn getppid() -> u32;
        #[link_name = "/poe/proc/exit__1"]
        pub fn exit(_: u32) -> !;
    }
}

#[inline]
pub fn yield_() {
    unsafe { raw::yield_() }
}

#[inline]
pub fn nanosleep(duration: u64) {
    unsafe { raw::nanosleep(duration) }
}

#[inline]
pub fn getpid() -> u32 {
    unsafe { raw::getpid() }
}

#[inline]
pub fn getppid() -> u32 {
    unsafe { raw::getppid() }
}

#[inline]
pub fn exit(rval_: u32) -> ! {
    unsafe { raw::exit(rval_) }
}
