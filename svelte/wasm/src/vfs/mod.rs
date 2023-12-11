mod infs;

use once_cell::sync::Lazy;

struct VFS {
    mountpoint: String,
    fs: Box<dyn VirtualFileSystem>
}
static mut MOUNTS: Lazy<Vec<VFS>> = Lazy::new(|| vec![]);

pub type VfsResult = Result<(), u8>;

pub trait VirtualFileSystem {
    fn get_fd(&mut self, inode: u32) -> Option<Box<dyn VirtualFileDescriptor>>;
    fn delete_file(&mut self, inode: u32) -> VfsResult;
    // returns the inode of the new file
    fn create_file(&mut self, dir_inode: u32, filename: String, data: Box<[u8]>) -> Option<u32>;
}

pub trait VirtualFileDescriptor {
    fn rewind_zero(&mut self) -> VfsResult;
    fn rewind(&mut self, count: u64) -> VfsResult;
    fn seek_forward(&mut self, count: u64) -> VfsResult;
    fn seek(&mut self, location: u64) -> VfsResult;
    fn read_to_eof(&mut self) -> Option<Box<[u8]>>;
    fn write_in_place(&mut self, buf: Box<[u8]>) -> VfsResult;
    // all non-in-place writes should, in good FSes, be COW
    fn overwrite(&mut self, buf: Box<[u8]>) -> VfsResult;
    fn append(&mut self, buf: Box<[u8]>) -> VfsResult; // append is especially important to be COW
    fn as_dentry(&mut self) -> Option<Box<dyn VirtualDentry>>;
}

pub trait VirtualDentry {
    fn get_entries(&mut self) -> Vec<VirtualDentryEntry>;
}

struct VirtualDentryEntry {
    inum: u32,
    filename: String
}
