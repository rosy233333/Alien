#![no_std]

#[macro_use]
pub mod console;
pub mod arch;
pub mod bus;
pub mod config;
pub mod io;
pub mod logging;
pub mod sync;
#[cfg(feature = "task")]
pub mod task;
pub mod time;
pub mod vm;

extern crate alloc;

pub use corelib::{
    backtrace, blk_crash_trick, check_kernel_space, create_domain, get_domain, kernel_satp,
    trap_from_user, trap_to_user, write_console,
};
