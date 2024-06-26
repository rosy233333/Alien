use alloc::{collections::BTreeMap, string::ToString, vec};

use core2::io::Read;
use interface::DomainTypeRaw;

use crate::domain_loader::creator::register_domain_elf;

const INIT_DOMAIN_LIST: &[(&str, DomainTypeRaw)] = &[
    ("buf_uart", DomainTypeRaw::BufUartDomain),
    ("buf_input", DomainTypeRaw::BufInputDomain),
    ("cache_blk", DomainTypeRaw::CacheBlkDeviceDomain),
    ("devfs", DomainTypeRaw::DevFsDomain),
    ("fatfs", DomainTypeRaw::FsDomain),
    ("goldfish", DomainTypeRaw::RtcDomain),
    ("null", DomainTypeRaw::EmptyDeviceDomain),
    ("pipefs", DomainTypeRaw::FsDomain),
    ("plic", DomainTypeRaw::PLICDomain),
    ("procfs", DomainTypeRaw::FsDomain),
    ("ramfs", DomainTypeRaw::FsDomain),
    ("random", DomainTypeRaw::EmptyDeviceDomain),
    ("shadow_blk", DomainTypeRaw::ShadowBlockDomain),
    ("syscall", DomainTypeRaw::SysCallDomain),
    ("sysfs", DomainTypeRaw::FsDomain),
    ("fifo_scheduler", DomainTypeRaw::SchedulerDomain),
    ("task", DomainTypeRaw::TaskDomain),
    ("vfs", DomainTypeRaw::VfsDomain),
    ("uart16550", DomainTypeRaw::UartDomain),
    ("virtio_mmio_block", DomainTypeRaw::BlkDeviceDomain),
    ("net_stack", DomainTypeRaw::NetDomain),
    ("logger", DomainTypeRaw::LogDomain),
];

pub fn init_domains() {
    let initrd = mem::INITRD_DATA.lock();
    if initrd.is_none() {
        panic!("Initrd data is not initialized");
    }
    let data = initrd.as_ref().unwrap();
    let mut decoder = libflate::gzip::Decoder::new(data.as_slice()).unwrap();
    let mut buf = vec![];
    let _r = decoder.read_to_end(&mut buf).unwrap();

    let mut map = BTreeMap::new();
    for entry in cpio_reader::iter_files(&buf) {
        let _mode = entry.mode();
        let name = entry.name();
        if name.starts_with("g") {
            let data = entry.file();
            let domain_name = name.split_once('g').unwrap().1;
            map.insert(domain_name.to_string(), data.to_vec());
        }
    }

    let mut register = |domain_file_name: &str, domain: DomainTypeRaw| {
        register_domain_elf(
            domain_file_name,
            map.remove(domain_file_name).unwrap(),
            domain,
        );
    };

    for (domain_file_name, domain) in INIT_DOMAIN_LIST {
        register(domain_file_name, *domain);
    }
}
