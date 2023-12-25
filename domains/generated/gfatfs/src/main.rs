#![feature(panic_info_message)]
#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use interface::{BlkDevice, Fs};
use libsyscall::{println, Syscall};
use rref::SharedHeap;

#[no_mangle]
fn main(
    sys: Box<dyn Syscall>,
    domain_id: u64,
    shared_heap: Box<dyn SharedHeap>,
    blk_device: Box<dyn BlkDevice>,
) -> Box<dyn Fs> {
    // init libsyscall
    libsyscall::init(sys, domain_id);
    rref::init(shared_heap);
    // call the real fatfs
    fatfs::main(blk_device)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no location information available");
    }
    libsyscall::backtrace();
    loop {}
}
