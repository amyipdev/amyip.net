// the IrisOS-Nano File System (INFS)
// known limitations:
// - contiguous only
// - this implementaion works exclusively in-memory
// - no journaling
// - no backup superblocks
// - for long-term use (not recommended) needs to be "defragged"
//   - thus very, very bad on SSDs (not a TRIM defrag)

use crate::vfs;

pub struct FileSystem {
    sup: Superblock,
    inode_use_cache: Box<[u8]>,
    inodes: Box<[Inode]>,
    data_use_table: Box<[u8]>,
    // since WASM is in-memory, we can afford to do this
    data: Box<[u8]>
}

enum FileSystemVersion {
    V1,
}

// superblock structure length is 32
struct Superblock {
    // determines version
    magic: u64,
    // total size of the filesystem, for info purposes
    data_size: u64,
    // must be a multiple of 8
    inode_count: u32,
    // must be a multiple of 256
    data_block_size: u32,
    // requires special checks
    block_count: u64,
    // determined by magic, do not store!
    version: FileSystemVersion
}

// inode structure length is 64
#[derive(Copy, Clone)]
struct Inode {
    // 0 = unused
    num: u32,
    first_block: u64,
    end_block: u64,
    // used to calculate position of EOF
    total_file_size: u64,
    // xxxx yyy rmx-rmx-rmx
    // type aug usr-grp-otr
    perms: u16,
    uid: u32,
    gid: u32,
    hard_link_count: u16,
    // these are timestamps
    accessed: u64,
    modified: u64,
    created: u64
}
#[derive(PartialEq)]
enum DirType {
    File = 0,
    Dir = 1,
    Symlink = 2,
    Hardlink = 3
}
impl Inode {
    fn get_dirtype(&self) -> DirType {
        match (self.perms & 0xf000) >> 12 {
            0 => DirType::File,
            1 => DirType::Dir,
            2 => DirType::Symlink,
            3 => DirType::Hardlink,
            _ => panic!("invalid dirtype")
        }
    }
}

struct FileDescriptor {
    // inode number
    inum: u32,
    // position
    pos: u64,
    // fd number
    fd: u32,
}
impl vfs::VirtualFileDescriptor for FileDescriptor {
    fn get_inum(&self) -> u32 {
        self.inum
    }
    fn get_pos(&self) -> u64 {
        self.pos
    }
    fn set_pos_raw(&mut self, pos: u64) -> vfs::VfsResult {
        self.pos = pos;
        Ok(())
    }
}

struct Dentry {
    intern: Vec<DentryEntry>,
    inum: u32
}
// TODO: don't push when inum = 0
impl Dentry {
    fn new(buf: &[u8], inum: u32) -> Self {
        if buf.len() % 256 != 0 {
            panic!("dentry is wrong length");
        }
        let mut res: Vec<DentryEntry> = vec![];
        for n in 0..buf.len()/256 {
            res.push(DentryEntry {
                inum: u32::from_le_bytes(buf[n*256..n*256+4].try_into().unwrap()),
                filename_cstr: buf[n*256+4..(n+1)*256].try_into().unwrap()
            })
        }
        Self {
            inum: inum,
            intern: res
        }
    }
}
impl vfs::VirtualDentry for Dentry {
    fn get_entries(&self) -> Vec<vfs::VirtualDentryEntry> {
        self.intern.iter().map(|x| x.convert_to_vfs()).collect()
    }
    fn get_inode(&self) -> u32 {
        self.inum
    }
}

struct DentryEntry {
    inum: u32,
    filename_cstr: [u8; 252]
}
impl DentryEntry {
    fn convert_to_vfs(&self) -> vfs::VirtualDentryEntry {
        vfs::VirtualDentryEntry {
            inum: self.inum,
            filename: std::str::from_utf8(&self.filename_cstr).unwrap().to_string()
        }
    }
}

impl FileSystem {
    fn check_inode(&self, inode: u32) -> bool {
        inode == 0 || inode >= self.sup.inode_count || self.inodes[inode as usize].num == 0
    }
    fn getcpos(&self, i: u32, p: u64) -> usize {
        (self.inodes[i as usize].first_block * (self.sup.data_block_size as u64) + p) as usize
    }
    // alloc_data vs alloc_inode: data needs a range, inode just needs one
    fn alloc_data(&mut self, blocks: u64) -> Option<u64> {
        let mut byte_start: u64 = 0;
        let mut bit_start: u8 = 0;
        let mut byte_end: u64 = 0;
        let mut bit_end: u8 = 0;
        let mut cached_dist: u64 = 0;
        while byte_end < crate::common::fastceildiv(self.sup.block_count, 8) {
            if self.data_use_table[byte_end as usize] & (1 << bit_end) == 0 {
                // this data is unused
                cached_dist += 1;
                if cached_dist == blocks {
                    return Some((byte_start << 3) + bit_start as u64);
                }
                if bit_end == 7 {
                    byte_end += 1;
                    bit_end = 0;
                } else {
                    bit_end += 1;
                }
            } else {
                if bit_end == 7 {
                    byte_end += 1;
                    bit_end = 0;
                } else {
                    bit_end += 1;
                }
                byte_start = byte_end;
                bit_start = bit_end;
                cached_dist = 0;
            }
        }
        None
    }
    pub fn create_test_fs() -> Self {
        Self {
            sup: Superblock {
                magic: 0x1815f05f7470ff65,
                data_size: 4194304,
                inode_count: 256,
                data_block_size: 4096,
                block_count: 1024,
                version: FileSystemVersion::V1
            },
            inode_use_cache: Box::new([0; 32]),
            inodes: Box::new([Inode {
                num: 0,
                first_block: 0,
                end_block: 0,
                total_file_size: 0,
                perms: 0,
                uid: 0,
                gid: 0,
                hard_link_count: 0,
                accessed: 0,
                modified: 0,
                created: 0
            }; 256]),
            data_use_table: Box::new([0; 128]),
            data: Box::new([0; 4194304])
        }
    }
}

impl vfs::VirtualFileSystem for FileSystem {
    fn get_fd(&mut self, inode: u32, fd: u32) -> Option<Box<dyn vfs::VirtualFileDescriptor>> {
        if !self.check_inode(inode) {
            return None;
        }
        // valid inode found, we can make the fd!
        Some(Box::new(FileDescriptor {
            inum: inode,
            pos: 0,
            fd: fd
        }))
    }
    fn delete_file(&mut self, inode: u32) -> vfs::VfsResult {unimplemented!()}
    fn create_file(&mut self, dir_inode: u32, filename: String, data: Box<[u8]>) -> Option<u32> {unimplemented!()}
    // we don't do anything special with fd's in INFS, so these are simple operations
    // however, other fs'es might cache information about where on disk to look (LBA-optimization)
    // that doesn't apply for this implementation though, but we need this flexibility
    fn rewind_zero(&mut self, fd: &mut Box::<dyn vfs::VirtualFileDescriptor>) -> vfs::VfsResult {
        if !self.check_inode(fd.get_inum()) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        // position 0 is always valid
        fd.set_pos_raw(0)
    }
    fn rewind(&mut self, fd: &mut Box::<dyn vfs::VirtualFileDescriptor>, count: u64) -> vfs::VfsResult {
        if !self.check_inode(fd.get_inum()) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        let cp: u64 = fd.get_pos();
        if count > cp {
            return Err(vfs::VfsErrno::EFPOOB);
        }
        fd.set_pos_raw(count - cp)
    }
    fn seek_forward(&mut self, fd: &mut Box::<dyn vfs::VirtualFileDescriptor>, count: u64) -> vfs::VfsResult {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        let cp: u64 = fd.get_pos();
        if count + cp >= self.inodes[i as usize].total_file_size {
            return Err(vfs::VfsErrno::EFPOOB);
        }
        fd.set_pos_raw(count + cp)
    }
    fn seek(&mut self, fd: &mut Box::<dyn vfs::VirtualFileDescriptor>, location: u64) -> vfs::VfsResult {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        if location >= self.inodes[i as usize].total_file_size {
            return Err(vfs::VfsErrno::EFPOOB);
        }
        fd.set_pos_raw(location)
    }
    fn read_n(&mut self, fd: &mut Box::<dyn vfs::VirtualFileDescriptor>, count: u64) -> Option<Vec<u8>> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        let p: u64 = fd.get_pos();
        if self.inodes[i as usize].total_file_size <= p + count {
            return None;
        }
        let a: usize = self.getcpos(i, p);
        fd.set_pos_raw(p+count).unwrap();
        Some(Vec::from(&self.data[a..a+(count as usize)]))
    }
    fn read_to_eof(&mut self, fd: &mut Box::<dyn vfs::VirtualFileDescriptor>) -> Option<Vec<u8>> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        self.read_n(fd, self.inodes[i as usize].total_file_size - fd.get_pos() - 1)
    }
    // TODO: factor out duplicate code
    fn write_in_place(&mut self, fd: &mut Box::<dyn vfs::VirtualFileDescriptor>, buf: &[u8]) -> vfs::VfsResult {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        let p: u64 = fd.get_pos();
        if self.inodes[i as usize].total_file_size <= p + (buf.len() as u64) {
            return Err(vfs::VfsErrno::EFPOOB);
        }
        let a: usize = self.getcpos(i, p);
        self.data[a..a+buf.len()].copy_from_slice(buf);
        Ok(())
    }
    fn overwrite(&mut self, fd: &mut Box::<dyn vfs::VirtualFileDescriptor>, buf: Box<[u8]>) -> vfs::VfsResult {unimplemented!()}
    fn append(&mut self, fd: &mut Box::<dyn vfs::VirtualFileDescriptor>, buf: Box<[u8]>) -> vfs::VfsResult {unimplemented!()}
    // TODO: factor out self.inodes[i as usize]
    fn vfd_as_dentry(&mut self, fd: &Box::<dyn vfs::VirtualFileDescriptor>) -> Option<Box<dyn vfs::VirtualDentry>> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        if self.inodes[i as usize].get_dirtype() != DirType::Dir {
            return None;
        }
        let sp = (self.inodes[i as usize].first_block*(self.sup.data_block_size as u64)) as usize;
        Some(Box::new(Dentry::new(&self.data[sp..sp+(self.inodes[i as usize].total_file_size as usize)], i)))
    }
}

// file system structure
// inode = 0 does not exist
// if a dentryentry refers to inode 0, that means nothing there
// V1's magic is 0x1815f05f7470ff65
//                 IRIS-OS-NANO--FS