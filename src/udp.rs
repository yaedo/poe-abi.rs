mod raw {
    use types::*;
    #[allow(improper_ctypes)]
    #[link(wasm_import_module = "/poe/udp")]
    extern "C" {
        #[link_name = "/poe/udp/open__1_0_0"]
        pub fn open(_: *const u8, _: usize, _: *mut fd) -> errno;
    }
}
use types::*;

#[inline]
pub unsafe fn open(addr_: &[u8], fd_: &mut fd) -> errno {
    raw::open(addr_.as_ptr(), addr_.len(), fd_)
}
