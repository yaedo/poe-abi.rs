mod raw {
    use types::*;
    #[allow(improper_ctypes)]
    #[link(wasm_import_module = "/poe/tcp")]
    extern "C" {
        #[link_name = "/poe/tcp/open__1"]
        pub fn open(_: *const u8, _: usize, _: *mut fd) -> errno;

    // TODO pooled
    }
}
use types::*;

#[inline]
pub unsafe fn open(addr_: &[u8], fd_: &mut fd) -> errno {
    raw::open(addr_.as_ptr(), addr_.len(), fd_)
}
