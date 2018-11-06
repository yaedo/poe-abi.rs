mod raw {
    use types::*;
    #[allow(improper_ctypes)]
    #[link(wasm_import_module = "/poe/resource")]
    extern "C" {
        #[link_name = "close__1"]
        pub fn close(_: fd) -> errno;
        #[link_name = "sync__1"]
        pub fn sync(_: fd) -> errno;
        #[link_name = "pread__1"]
        pub fn pread(_: fd, _: *const iovec, _: usize, _: filesize, _: *mut usize) -> errno;
        #[link_name = "pwrite__1"]
        pub fn pwrite(_: fd, _: *const ciovec, _: usize, _: filesize, _: *mut usize) -> errno;
        #[link_name = "read__1"]
        pub fn read(_: fd, _: *const iovec, _: usize, _: *mut usize) -> errno;
        #[link_name = "write__1"]
        pub fn write(_: fd, _: *const ciovec, _: usize, _: *mut usize) -> errno;
        #[link_name = "seek__1"]
        pub fn seek(_: fd, _: filedelta, _: whence, _: *mut filesize) -> errno;
        #[link_name = "stat_get__1"]
        pub fn stat_get(_: fd, _: *mut fdstat) -> errno;
        #[link_name = "stat_put__1"]
        pub fn stat_put(_: fd, _: *const fdstat, _: fdsflags) -> errno;
        #[link_name = "unlink__1"]
        pub fn unlink(_: fd, _: ulflags) -> errno;
    // TODO do we need a pipe?
    }
}
use types::*;

#[inline]
pub unsafe fn close(fd_: fd) -> errno {
    raw::close(fd_)
}

#[inline]
pub unsafe fn sync(fd_: fd) -> errno {
    raw::sync(fd_)
}

#[inline]
pub unsafe fn pread(fd_: fd, iovs_: &[iovec], offset_: filesize, nread_: &mut usize) -> errno {
    raw::pread(fd_, iovs_.as_ptr(), iovs_.len(), offset_, nread_)
}

#[inline]
pub unsafe fn pwrite(fd_: fd, iovs_: &[ciovec], offset_: filesize, nwritten_: &mut usize) -> errno {
    raw::pwrite(fd_, iovs_.as_ptr(), iovs_.len(), offset_, nwritten_)
}

#[inline]
pub unsafe fn read(fd_: fd, iovs_: &[iovec], nread_: &mut usize) -> errno {
    raw::read(fd_, iovs_.as_ptr(), iovs_.len(), nread_)
}

#[inline]
pub unsafe fn seek(
    fd_: fd,
    offset_: filedelta,
    whence_: whence,
    newoffset_: &mut filesize,
) -> errno {
    raw::seek(fd_, offset_, whence_, newoffset_)
}

#[inline]
pub unsafe fn stat_get(fd_: fd, buf_: *mut fdstat) -> errno {
    raw::stat_get(fd_, buf_)
}

#[inline]
pub unsafe fn stat_put(fd_: fd, buf_: *const fdstat, flags_: fdsflags) -> errno {
    raw::stat_put(fd_, buf_, flags_)
}

#[inline]
pub unsafe fn write(fd_: fd, iovs_: &[ciovec], nwritten_: &mut usize) -> errno {
    raw::write(fd_, iovs_.as_ptr(), iovs_.len(), nwritten_)
}

#[inline]
pub unsafe fn unlink(fd_: fd, flags_: ulflags) -> errno {
    raw::unlink(fd_, flags_)
}
