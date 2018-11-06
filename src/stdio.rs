mod raw {
    use types::*;
    #[link(wasm_import_module = "/poe/stdio")]
    extern "C" {
        #[link_name = "/poe/stdio/in__1"]
        pub fn stdin(fd_: *mut fd) -> errno;
        #[link_name = "/poe/stdio/out__1"]
        pub fn stdout(fd_: *mut fd) -> errno;
        #[link_name = "/poe/stdio/err__1"]
        pub fn stderr(fd_: *mut fd) -> errno;
    }
}
use types::*;

#[inline]
pub unsafe fn stdin(fd_: &mut fd) -> errno {
    raw::stdin(fd_)
}

#[inline]
pub unsafe fn stdout(fd_: &mut fd) -> errno {
    raw::stdout(fd_)
}

#[inline]
pub unsafe fn stderr(fd_: &mut fd) -> errno {
    raw::stderr(fd_)
}
