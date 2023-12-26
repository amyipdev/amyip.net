pub(crate) mod infs;

//use once_cell::sync::Lazy;

use std::collections::HashMap;

// could lazy load the rootfs
// mount.root downloads the standard rootfs, mounts on /
// mount.web downloads an alternative fs, mounts wherever

// TODO: permissions checks

// TODO: procfs/sysfs/devfs (base: chardevfs)

// in a MultiMount, "." contains the rootfs
enum VfsTreeNode {
    Mounted(Box<dyn VirtualFileSystem>),
    MultiMount(HashMap<String, VfsTreeNode>),
    Unmounted
}
impl VfsTreeNode {
    // input should be form of "path/to/filesys/file"
    // output string is path to file on the filesys
    fn find_destination_fs(&mut self, path: String) -> (&mut Box<dyn VirtualFileSystem>, String) {
        match self {
            VfsTreeNode::Mounted(ref mut n) => (n, path),
            VfsTreeNode::MultiMount(ref mut n) => {
                let mut rempath: Vec<String> = path.splitn(2, '/').map(|x| x.to_string()).collect();
                if rempath.len() == 1 {
                    rempath.insert(0, ".".to_string());
                }
                if n.contains_key(&rempath[0]) {
                    return n.get_mut(&rempath[0]).unwrap().find_destination_fs(rempath[1].clone());
                }
                (n.get_mut(".").unwrap().presume_mounted().unwrap(), rempath[1].clone())
            }
            _ => panic!("No FS mounted!")
        }
    }
    fn presume_mounted(&mut self) -> Option<&mut Box<dyn VirtualFileSystem>> {
        match self {
            VfsTreeNode::Mounted(n) => Some(n),
            _ => None
        }
    }
}
static mut VFS_ROOT: VfsTreeNode = VfsTreeNode::Unmounted;
pub fn mount_root(fs: Box<dyn VirtualFileSystem>) {
    unsafe {
        VFS_ROOT = VfsTreeNode::Mounted(fs);
    }
}
pub fn safe_wrap_fdfs<'a>(path: String) -> (&'a mut Box<dyn VirtualFileSystem>, String) {
    unsafe {
        VFS_ROOT.find_destination_fs(path)
    }
}

pub enum VfsErrno {
    EINVFD,
    EFPOOB,
    ENSTOR,
}
impl VfsErrno {
    pub fn errno(&self) -> &str {
        match self {
            VfsErrno::EINVFD => "file descriptor points to nonexistent file",
            VfsErrno::EFPOOB => "file seek went out of bounds",
            VfsErrno::ENSTOR => "not enough space on disk",
        }
    }
}
impl std::fmt::Debug for VfsErrno {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.errno())
    }
}
pub type VfsResult = Result<(), VfsErrno>;

pub trait VirtualFileSystem {
    fn get_fd(&self, inode: u32, fd: u32) -> Option<Box<dyn VirtualFileDescriptor>>;
    fn delete_file(&mut self, inode: u32, dir_inode: u32) -> VfsResult;
    // returns the inode of the new file
    fn create_file(&mut self, dir_inode: u32, filename: String, data: &[u8]) -> Option<u32>;
    fn rewind_zero(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>) -> VfsResult;
    fn rewind(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, count: u64) -> VfsResult;
    fn seek_forward(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, count: u64) -> VfsResult;
    fn seek(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, location: u64) -> VfsResult;
    fn read_n(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, count: u64) -> Option<Vec<u8>>;
    fn read_to_eof(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>) -> Option<Vec<u8>>;
    fn write_in_place(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, buf: &[u8]) -> VfsResult;
    // all non-in-place writes should, in good FSes, be COW
    fn overwrite(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, buf: &[u8]) -> VfsResult;
    fn append(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, buf: &[u8]) -> VfsResult; // append is especially important to be COW
    fn vfd_as_dentry(
        &mut self,
        fd: &Box<dyn VirtualFileDescriptor>,
    ) -> Option<Box<dyn VirtualDentry>>;
}

pub trait VirtualFileDescriptor {
    fn get_inum(&self) -> u32;
    fn get_pos(&self) -> u64;
    // set_pos_raw does no checking, should only be called through VirtualFileSystem functions
    fn set_pos_raw(&mut self, pos: u64) -> VfsResult;
}

pub trait VirtualDentry {
    fn get_entries(&self) -> Vec<VirtualDentryEntry>;
    fn get_inode(&self) -> u32;
}

struct VirtualDentryEntry {
    inum: u32,
    filename: String,
}
