#![feature(allocator_api)]
#![no_std]
#![crate_type = "rlib"]

use core::alloc::{AllocErr, GlobalAlloc, Layout};
use core::ptr::NonNull;

#[link(wasm_import_module = "poe_alloc")]
extern "C" {
    fn alloc(size: usize, align: usize) -> usize;
    fn dealloc(size: usize, align: usize, ptr: *mut u8);
}

pub struct PoeAlloc {}

unsafe fn alloc_impl(layout: Layout) -> Result<NonNull<u8>, AllocErr> {
    let res = alloc(layout.size(), layout.align());
    if res == 0 {
        Err(AllocErr)
    } else {
        if let Some(ptr) = NonNull::new(res as *mut u8) {
            Ok(ptr)
        } else {
            Err(AllocErr)
        }
    }
}

unsafe fn dealloc_impl(ptr: NonNull<u8>, layout: Layout) {
    dealloc(layout.size(), layout.align(), ptr.as_ptr());
}

unsafe impl Sync for PoeAlloc {}

#[cfg(feature = "nightly")]
unsafe impl<'a, 'b> Alloc for &'b PoeAlloc<'a>
where
    'a: 'b,
{
    unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        alloc_impl(layout)
    }

    unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        dealloc_impl(ptr, layout)
    }
}

unsafe impl GlobalAlloc for PoeAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match alloc_impl(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(AllocErr) => 0 as *mut u8,
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(ptr) = NonNull::new(ptr) {
            dealloc_impl(ptr, layout);
        }
    }
}
