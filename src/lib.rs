#[macro_use]
extern crate lazy_static;
extern crate log as log_crate;
extern crate poe_alloc;

pub mod allocator {
    pub use poe_alloc::PoeAlloc;
    pub const POE_ALLOC: PoeAlloc = PoeAlloc {};
}
pub mod env;
pub mod io;
pub mod log;
mod resource;

pub use resource::Resource;
