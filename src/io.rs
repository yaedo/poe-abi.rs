use std::cell::RefCell;

use resource::Resource;

thread_local! {
    static STDIN: RefCell<Resource> = RefCell::new(
        Resource::open(b"std://in").unwrap()
    );
    static STDOUT: RefCell<Resource> = RefCell::new(
        Resource::open(b"std://out").unwrap()
    );
    static STDERR: RefCell<Resource> = RefCell::new(
        Resource::open(b"std://err").unwrap()
    );
}

pub fn with_stdin<F: FnOnce(&mut Resource) -> T, T>(f: F) -> T {
    STDIN.with(|h| f(&mut *h.borrow_mut()))
}

pub fn with_stdout<F: FnOnce(&mut Resource) -> T, T>(f: F) -> T {
    STDOUT.with(|h| f(&mut *h.borrow_mut()))
}

pub fn with_stderr<F: FnOnce(&mut Resource) -> T, T>(f: F) -> T {
    STDERR.with(|h| f(&mut *h.borrow_mut()))
}
