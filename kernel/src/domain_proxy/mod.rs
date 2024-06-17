mod scheduler;
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{any::Any, fmt::Debug, mem::forget, net::SocketAddrV4, ops::Range};

use downcast_rs::{impl_downcast, DowncastSync};
use interface::*;
use ksync::Mutex;
use pconst::{
    io::{PollEvents, RtcTime, SeekFrom},
    net::*,
};
use rref::{RRef, RRefVec, SharedData};
pub use scheduler::SchedulerDomainProxy;
use spin::Once;
use vfscore::{fstype::FileSystemFlags, inode::InodeAttr, superblock::SuperType, utils::*};

use crate::{
    domain_helper::{alloc_domain_id, free_domain_resource},
    domain_loader::loader::DomainLoader,
    error::{AlienError, AlienResult},
    sync::{RcuData, SRcuLock},
    task::continuation,
};

pub trait ProxyBuilder {
    type T;
    fn build(domain: Self::T, domain_loader: DomainLoader) -> Self;
    fn build_empty(domain_loader: DomainLoader) -> Self;
    fn init_by_box(&self, argv: Box<dyn Any + Send + Sync>) -> AlienResult<()>;
}

gen_for_BufInputDomain!();
gen_for_BufUartDomain!();
gen_for_CacheBlkDeviceDomain!();
gen_for_EmptyDeviceDomain!();
gen_for_FsDomain!();
gen_for_GpuDomain!();
gen_for_InputDomain!();
gen_for_NetDeviceDomain!();
gen_for_RtcDomain!();
gen_for_SysCallDomain!();
gen_for_TaskDomain!();
gen_for_UartDomain!();
gen_for_VfsDomain!();
gen_for_PLICDomain!();
// gen_for_SchedulerDomain!();
gen_for_ShadowBlockDomain!();
gen_for_BlkDeviceDomain!();

gen_for_DevFsDomain!();
gen_for_LogDomain!();
gen_for_NetDomain!();
impl_for_FsDomain!(DevFsDomainProxy);

impl_empty_for_FsDomain!(DevFsDomainEmptyImpl);
impl Basic for DevFsDomainEmptyImpl {
    fn domain_id(&self) -> u64 {
        u64::MAX
    }
    fn is_active(&self) -> bool {
        false
    }
}
impl Basic for DevFsDomainProxy {
    fn domain_id(&self) -> u64 {
        let idx = self.srcu_lock.read_lock();
        let res = self.domain.get().domain_id();
        self.srcu_lock.read_unlock(idx);
        res
    }
    fn is_active(&self) -> bool {
        let idx = self.srcu_lock.read_lock();
        let res = self.domain.get().is_active();
        self.srcu_lock.read_unlock(idx);
        res
    }
}

pub trait ProxyExt: DowncastSync {
    fn reload(&self) -> AlienResult<()>;
}

impl_downcast!(sync ProxyExt);
impl ShadowBlockDomainProxy {
    pub fn replace(
        &self,
        new_domain: Box<dyn ShadowBlockDomain>,
        loader: DomainLoader,
    ) -> AlienResult<()> {
        let mut loader_guard = self.domain_loader.lock();
        let old_id = self.domain_id();
        let old_domain = self.domain.swap(Box::new(new_domain));
        // synchronize the reader which is reading the old domain
        // println!("srcu synchronize");
        self.srcu_lock.synchronize();
        // println!("srcu synchronize end");

        // forget the old domain
        // it will be dropped by the `free_domain_resource`
        let real_domain = Box::into_inner(old_domain);
        forget(real_domain);

        free_domain_resource(old_id);

        let resource = self.resource.get().unwrap();
        let info = resource.as_ref().downcast_ref::<String>().unwrap();
        self.domain.get().init(info).unwrap();

        *loader_guard = loader;
        Ok(())
    }
}

impl ProxyExt for BlkDomainProxy {
    fn reload(&self) -> AlienResult<()> {
        let mut loader_guard = self.domain_loader.lock();
        let mut loader = loader_guard.clone();
        loader.load().unwrap();
        let old_id = self.domain_id();
        let new_id = alloc_domain_id();

        let new_domain = loader.call(new_id);
        let old_domain = self.domain.swap(Box::new(new_domain));
        // synchronize the reader which is reading the old domain
        self.srcu_lock.synchronize();

        // forget the old domain
        let real_domain = Box::into_inner(old_domain);
        forget(real_domain);

        free_domain_resource(old_id);

        let device_info = self.resource.get().unwrap();
        let info = device_info.as_ref().downcast_ref::<Range<usize>>().unwrap();
        self.domain.get().init(info).unwrap();

        *loader_guard = loader;
        Ok(())
    }
}

impl LogDomainProxy {
    pub fn replace(&self, new_domain: Box<dyn LogDomain>, loader: DomainLoader) -> AlienResult<()> {
        let mut loader_guard = self.domain_loader.lock();
        let old_id = self.domain_id();
        let old_domain = self.domain.swap(Box::new(new_domain));
        // synchronize the reader which is reading the old domain
        self.srcu_lock.synchronize();

        // forget the old domain
        let real_domain = Box::into_inner(old_domain);
        forget(real_domain);

        // free the old domain resource
        free_domain_resource(old_id);

        self.domain.get().init().unwrap();
        *loader_guard = loader;
        Ok(())
    }
}

impl GpuDomainProxy {
    pub fn replace(&self, new_domain: Box<dyn GpuDomain>, loader: DomainLoader) -> AlienResult<()> {
        let mut loader_guard = self.domain_loader.lock();
        let old_id = self.domain_id();
        let old_domain = self.domain.swap(Box::new(new_domain));
        // synchronize the reader which is reading the old domain
        self.srcu_lock.synchronize();

        // forget the old domain
        let real_domain = Box::into_inner(old_domain);
        forget(real_domain);

        // free the old domain resource
        free_domain_resource(old_id);

        let device_info = self.resource.get().unwrap();
        let info = device_info.as_ref().downcast_ref::<Range<usize>>().unwrap();
        self.domain.get().init(info).unwrap();

        *loader_guard = loader;
        Ok(())
    }
}

impl InputDomainProxy {
    pub fn replace(
        &self,
        new_domain: Box<dyn InputDomain>,
        loader: DomainLoader,
    ) -> AlienResult<()> {
        let mut loader_guard = self.domain_loader.lock();
        let old_id = self.domain_id();
        let old_domain = self.domain.swap(Box::new(new_domain));
        // synchronize the reader which is reading the old domain
        self.srcu_lock.synchronize();

        // forget the old domain
        let real_domain = Box::into_inner(old_domain);
        forget(real_domain);

        // free the old domain resource
        free_domain_resource(old_id);

        let device_info = self.resource.get().unwrap();
        let info = device_info.as_ref().downcast_ref::<Range<usize>>().unwrap();
        self.domain.get().init(info).unwrap();

        *loader_guard = loader;
        Ok(())
    }
}

impl NetDeviceDomainProxy {
    pub fn replace(
        &self,
        new_domain: Box<dyn NetDeviceDomain>,
        loader: DomainLoader,
    ) -> AlienResult<()> {
        let mut loader_guard = self.domain_loader.lock();
        let old_id = self.domain_id();
        let old_domain = self.domain.swap(Box::new(new_domain));
        // synchronize the reader which is reading the old domain
        self.srcu_lock.synchronize();

        // forget the old domain
        let real_domain = Box::into_inner(old_domain);
        forget(real_domain);

        // free the old domain resource
        free_domain_resource(old_id);

        let device_info = self.resource.get().unwrap();
        let info = device_info.as_ref().downcast_ref::<Range<usize>>().unwrap();
        self.domain.get().init(info).unwrap();

        *loader_guard = loader;
        Ok(())
    }
}
