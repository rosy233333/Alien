use core::any::Any;

use alloc::boxed::Box;
use constants::AlienResult;
use rref::RRefVec;

use crate::{DeviceBase, DeviceInfo};

pub trait NetDomain: DeviceBase {
    fn init(&self, device_info: &DeviceInfo) -> AlienResult<()>;
    // fn medium(&self) -> Medium;
    
    /// The ethernet address of the NIC.
    fn mac_address(&self) -> AlienResult<[u8;6]>;

    /// Whether can transmit packets.
    fn can_transmit(&self) -> AlienResult<bool>;

    /// Whether can receive packets.
    fn can_receive(&self) -> AlienResult<bool>;

    /// Size of the receive queue.
    fn rx_queue_size(&self) -> AlienResult<usize>;

    /// Size of the transmit queue.
    fn tx_queue_size(&self) -> AlienResult<usize>;

    /// Gives back the `rx_buf` to the receive queue for later receiving.
    ///
    /// `rx_buf` should be the same as the one returned by
    /// [`NetDriverOps::receive`].
    fn recycle_rx_buffer(&self, rx_buf: NetBuf) -> AlienResult<()>;

    /// Poll the transmit queue and gives back the buffers for previous transmiting.
    /// returns [`DevResult`].
    fn recycle_tx_buffers(&self) -> AlienResult<()>;

    /// Transmits a packet in the buffer to the network, without blocking,
    /// returns [`DevResult`].
    fn transmit(&self, net_buf: NetBuf) -> AlienResult<()>;

    /// Receives a packet from the network and store it in the [`NetBuf`],
    /// returns the buffer.
    ///
    /// Before receiving, the driver should have already populated some buffers
    /// in the receive queue by [`NetDriverOps::recycle_rx_buffer`].
    ///
    /// If currently no incomming packets, returns an error with type
    /// [`DevError::Again`].
    fn receive(&self) -> AlienResult<NetBuf>;

    /// Allocate a memory buffer of a specified size for network transmission,
    /// returns [`DevResult`]
    fn alloc_tx_buffer(&self, size: usize) -> AlienResult<NetBuf>;
}

pub struct NetBuf {
    pub data: RRefVec<u8>,
    pub net_buf: Box<dyn Any>
}