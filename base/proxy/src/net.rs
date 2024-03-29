use alloc::boxed::Box;

use constants::{AlienError, AlienResult};
use interface::{Basic, DeviceBase, DeviceInfo, NetDomain, RxBufferWrapper, TxBufferWrapper};

#[derive(Debug)]
pub struct NetDomainProxy {
    id: u64,
    domain: Box<dyn NetDomain>,
}

impl NetDomainProxy {
    pub fn new(id: u64, domain: Box<dyn NetDomain>) -> Self {
        Self { id, domain }
    }
}

impl NetDomain for NetDomainProxy {
    fn init(&self, device_info: &DeviceInfo) -> AlienResult<()> {
        self.domain.init(device_info)
    }

    fn mac_address(&self) -> AlienResult<[u8; 6]> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.mac_address()
    }

    fn can_transmit(&self) -> AlienResult<bool> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.can_transmit()
    }

    fn can_receive(&self) -> AlienResult<bool> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.can_receive()
    }

    fn rx_queue_size(&self) -> AlienResult<usize> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.rx_queue_size()
    }

    fn tx_queue_size(&self) -> AlienResult<usize> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.tx_queue_size()
    }

    fn recycle_rx_buffer(&self, rx_buf: RxBufferWrapper) -> AlienResult<()> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.recycle_rx_buffer(rx_buf)
    }

    fn recycle_tx_buffers(&self) -> AlienResult<()> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.recycle_tx_buffers()
    }

    fn transmit(&self, tx_buf: TxBufferWrapper) -> AlienResult<()> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.transmit(tx_buf)
    }

    fn receive(&self) -> AlienResult<RxBufferWrapper> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.receive()
    }

    fn alloc_tx_buffer(&self, size: usize) -> AlienResult<TxBufferWrapper> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.alloc_tx_buffer(size)
    }
}

impl DeviceBase for NetDomainProxy {
    fn handle_irq(&self) -> AlienResult<()> {
        if !self.is_active() {
            return Err(AlienError::DOMAINCRASH);
        }
        self.domain.handle_irq()
    }
}

impl Basic for NetDomainProxy {
    fn is_active(&self) -> bool {
        self.domain.is_active()
    }
}
