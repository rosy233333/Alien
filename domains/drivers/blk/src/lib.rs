//! This crate should implement the block device driver according to the VirtIO specification.
//! The [virtio-blk](virtio_blk) crate provides the safety abstraction for the VirtIO registers and buffers.
//! So this crate should only implement the driver logic with safe Rust code.
#![no_std]
#![deny(unsafe_code)]
extern crate alloc;
extern crate malloc;

use alloc::sync::Arc;
use core::fmt::Debug;
use interface::Basic;
use ksync::Mutex;
use libsyscall::println;
use log::info;
use rref::RpcResult;
use virtio_blk::VirtIoBlk;

pub struct VirtIOBlk {
    driver: Arc<Mutex<VirtIoBlk>>,
}

impl Debug for VirtIOBlk {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VirtIOBlk")
    }
}

impl VirtIOBlk {
    pub fn new(virtio_blk_addr: usize) -> Self {
        Self {
            driver: Arc::new(Mutex::new(VirtIoBlk::new(virtio_blk_addr))),
        }
    }
}

impl Drop for VirtIOBlk {
    fn drop(&mut self) {
        info!("Drop VirtIOBlk");
    }
}

impl Basic for VirtIOBlk {
    // fn drop_self(self: Box<Self>) {
    //     info!("Drop VirtIOBlk");
    //     drop(self);
    // }
}

impl interface::BlkDevice for VirtIOBlk {
    fn read(&self, block: u32, data: rref::RRef<[u8; 512]>) -> RpcResult<rref::RRef<[u8; 512]>> {
        let mut buf = data;
        self.driver
            .lock()
            .read_block(block as usize, buf.as_mut())
            .unwrap();
        // warn!("read block: {}, buf:{:#x}", block, buf[0]);
        // panic!("read block: {}, buf:{:#x}", block, buf[0]);
        Ok(buf)
    }
    fn write(&self, block: u32, data: &rref::RRef<[u8; 512]>) -> RpcResult<usize> {
        self.driver
            .lock()
            .write_block(block as usize, data.as_ref())
            .unwrap();
        Ok(data.len())
    }

    fn get_capacity(&self) -> RpcResult<u64> {
        println!("get_capacity: {}", self.driver.lock().capacity() as u64);
        Ok(self.driver.lock().capacity() as u64)
    }

    fn flush(&self) -> RpcResult<()> {
        Ok(())
    }

    fn handle_irq(&self) -> RpcResult<()> {
        unimplemented!()
    }
}

pub fn main(virtio_blk_addr: usize) -> Arc<dyn interface::BlkDevice> {
    println!("virtio_blk_addr: {:#x}", virtio_blk_addr);
    Arc::new(VirtIOBlk::new(virtio_blk_addr))
}
