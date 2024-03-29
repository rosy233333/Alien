use constants::AlienResult;
use rref::{RRef, RRefVec};
use vfscore::utils::{VfsFileStat, VfsFsStat, VfsNodeType};

use crate::Basic;

pub type InodeID = u64;
pub const VFS_ROOT_ID: InodeID = 0;
pub const VFS_STDIN_ID: InodeID = 1;
pub const VFS_STDOUT_ID: InodeID = 2;
pub const VFS_STDERR_ID: InodeID = 3;

pub struct DirEntryWrapper {
    /// ino is an inode number
    pub ino: u64,
    /// type is the file type
    pub ty: VfsNodeType,
    /// filename (null-terminated)
    pub name: RRefVec<u8>,
    pub name_len: usize,
}

impl DirEntryWrapper {
    pub fn new(name: RRefVec<u8>) -> Self {
        Self {
            ino: 0,
            ty: VfsNodeType::Unknown,
            name,
            name_len: 0,
        }
    }
}

pub trait VfsDomain: Basic {
    fn init(&self) -> AlienResult<()>;
    fn vfs_open(
        &self,
        root: InodeID,
        path: &RRefVec<u8>,
        mode: u32,
        open_flags: usize,
    ) -> AlienResult<InodeID>;
    fn vfs_close(&self, inode: InodeID) -> AlienResult<()>;
    fn vfs_getattr(
        &self,
        inode: InodeID,
        attr: RRef<VfsFileStat>,
    ) -> AlienResult<RRef<VfsFileStat>>;
    fn vfs_read_at(
        &self,
        inode: InodeID,
        offset: u64,
        buf: RRefVec<u8>,
    ) -> AlienResult<(RRefVec<u8>, usize)>;

    fn vfs_read(&self, inode: InodeID, buf: RRefVec<u8>) -> AlienResult<(RRefVec<u8>, usize)>;

    fn vfs_write_at(
        &self,
        inode: InodeID,
        offset: u64,
        buf: RRefVec<u8>,
    ) -> AlienResult<(RRefVec<u8>, usize)>;
    fn vfs_write(&self, inode: InodeID, buf: &RRefVec<u8>) -> AlienResult<usize>;
}
