mod raw {
    #[link(wasm_import_module = "/poe/rand")]
    extern "C" {
        // TODO read random bytes

        #[link_name = "/poe/rand/u32__1"]
        pub fn u32() -> u32;
        #[link_name = "/poe/rand/u64__1"]
        pub fn u64() -> u64;
    }
}

#[inline]
pub fn u32() -> u32 {
    unsafe { raw::u32() }
}

#[inline]
pub fn u64() -> u64 {
    unsafe { raw::u64() }
}
