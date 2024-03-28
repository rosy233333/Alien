#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]
#[macro_use]
pub mod console;
mod basic;
mod common_riscv;
mod logger;
mod qemu_riscv;

pub use basic::MachineInfo as PlatformInfo;
use qemu_riscv::console_putchar;
pub use qemu_riscv::{config, set_timer, system_shutdown};
use spin::Once;

use crate::common_riscv::sbi::hart_start;

extern "C" {
    fn sbss();
    fn ebss();
}

/// 清空.bss段
fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

pub fn platform_init(hart_id: usize, dtb: usize) {
    clear_bss();
    println!("{}", ::config::ALIEN_FLAG);
    qemu_riscv::init_dtb(Some(dtb));
    let machine_info = basic::machine_info_from_dtb(platform_dtb_ptr());
    MACHINE_INFO.call_once(|| machine_info);
    logger::init_logger();
    #[cfg(feature = "smp")]
    init_other_hart(hart_id);
    unsafe { main(hart_id) }
}

fn init_other_hart(hart_id: usize) {
    let start_hart = 0;
    for i in start_hart..::config::CPU_NUM {
        if i != hart_id {
            let res = hart_start(i, _start_secondary as usize, 0);
            assert_eq!(res.error, 0);
        }
    }
}

extern "C" {
    fn main(hart_id: usize);
    fn _start_secondary();
}

pub fn platform_dtb_ptr() -> usize {
    return *qemu_riscv::DTB.get().unwrap();
}

static MACHINE_INFO: Once<PlatformInfo> = Once::new();

pub fn platform_machine_info() -> PlatformInfo {
    MACHINE_INFO.get().unwrap().clone()
}
