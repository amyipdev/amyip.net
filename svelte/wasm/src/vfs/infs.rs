// the IrisOS-Nano File System (INFS)
// known limitations:
// - contiguous only
// - this implementaion works exclusively in-memory
// - no journaling
// - no backup superblocks
// - for long-term use (not recommended) needs to be "defragged"
//   - thus very, very bad on SSDs (not a TRIM defrag)

// TODO: actually implement symlinks and hardlinks
//   - this means lots of dealing with reads/dentry work

use crate::vfs;
use crate::vfs::VirtualFileSystem;
use std::sync::atomic::Ordering;

pub struct FileSystem {
    sup: Superblock,
    inode_use_cache: Box<[u8]>,
    inodes: Box<[Inode]>,
    data_use_table: Box<[u8]>,
    // since WASM is in-memory, we can afford to do this
    data: Box<[u8]>,
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
    version: FileSystemVersion,
}

// inode structure length is 64
// TODO: consider implementing Default
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
    created: u64,
}
#[derive(PartialEq)]
enum DirType {
    File = 0,
    Dir = 1,
    Symlink = 2,
    // this may need to be removed
    // hardlinks just steal the inum
    Hardlink = 3,
}
impl Inode {
    fn get_dirtype(&self) -> DirType {
        match self.perms >> 12 {
            0 => DirType::File,
            1 => DirType::Dir,
            2 => DirType::Symlink,
            3 => DirType::Hardlink,
            _ => panic!("invalid dirtype"),
        }
    }
}

struct FileDescriptor {
    // inode number
    inum: u32,
    // position
    pos: u64,
    // REMOVED: fd number
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
    inum: u32,
}
// TODO: don't push when inum = 0
// TODO: store dentry in sorted order, binary search dentry contents
impl Dentry {
    fn new(buf: &[u8], inum: u32) -> Self {
        if buf.len() % 256 != 0 {
            panic!("dentry is wrong length");
        }
        let mut res: Vec<DentryEntry> = vec![];
        for n in 0..buf.len() / 256 {
            res.push(DentryEntry {
                inum: u32::from_le_bytes(buf[n * 256..n * 256 + 4].try_into().unwrap()),
                filename_cstr: buf[n * 256 + 4..(n + 1) * 256].try_into().unwrap(),
            });
            if let Some(j) = res.last() {
                if j.inum == 0 {
                    res.remove(res.len() - 1);
                }
            }
        }
        Self {
            inum: inum,
            intern: res,
        }
    }
    // alternative to vfd_as_dentry to access local elements
    fn from_internal(inum: u32, fs: &FileSystem) -> Option<Self> {
        if !fs.check_inode(inum) {
            return None;
        }
        if fs.inodes[inum as usize].get_dirtype() != DirType::Dir {
            return None;
        }
        let dp: usize =
            (fs.inodes[inum as usize].first_block * (fs.sup.data_block_size as u64)) as usize;
        Some(Dentry::new(
            &fs.data[dp..(dp + fs.inodes[inum as usize].total_file_size as usize)],
            inum,
        ))
    }
    fn write_back(self, fs: &mut FileSystem) {
        let mut ba: Vec<u8> = vec![];
        for item in self.intern {
            ba.extend(u32::to_le_bytes(item.inum));
            ba.extend(item.filename_cstr);
        }
        ba.extend(
            std::iter::repeat(0u8).take(
                fs.sup.data_block_size as usize - (ba.len() % fs.sup.data_block_size as usize),
            ),
        );
        fs.overwrite(&mut fs.get_fd(self.inum, 0x0).unwrap(), &ba)
            .unwrap();
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
    filename_cstr: [u8; 252],
}
impl DentryEntry {
    fn convert_to_vfs(&self) -> vfs::VirtualDentryEntry {
        vfs::VirtualDentryEntry {
            inum: self.inum,
            filename: crate::common::bytes_to_string(&self.filename_cstr),
        }
    }
}

impl FileSystem {
    fn clear_inode(&mut self, inode: u32) -> vfs::VfsResult {
        self.inode_use_cache[(inode >> 3) as usize] &= !(1 << (inode % 8));
        self.inodes[inode as usize].num = 0;
        Ok(())
    }
    fn clear_data(&mut self, sb: u64, eb: u64) -> vfs::VfsResult {
        let sby = sb >> 3;
        let sbi = sb % 8;
        let eby = eb >> 3;
        let ebi = eb % 8;
        if sby - eby > 1 {
            for n in sby + 1..eby {
                self.data_use_table[n as usize] &= 0x0;
            }
        }
        for n in sbi..8 {
            self.data_use_table[sby as usize] &= !(1 << n);
        }
        for n in 0..ebi + 1 {
            self.data_use_table[eby as usize] &= !(1 << n);
        }
        let s: u64 = self.sup.data_block_size as u64;
        // secure erase the blocks since this is in-memory
        for b in &mut self.data[((sb * s) as usize)..(((eb + 1) * s) as usize)] {
            *b = 0;
        }
        Ok(())
    }
    fn check_inode(&self, inode: u32) -> bool {
        !(inode == 0 || inode >= self.sup.inode_count || self.inodes[inode as usize].num == 0)
    }
    fn getcpos(&self, i: u32, p: u64) -> usize {
        (self.inodes[i as usize].first_block * (self.sup.data_block_size as u64) + p) as usize
    }
    // alloc_data vs alloc_inode: data needs a range, inode just needs one
    fn alloc_inode(&mut self) -> Option<u32> {
        let mut cby: u64 = 0;
        let mut cbi: u8 = 1;
        let bc: u64 = crate::common::fastceildiv(self.sup.inode_count as u64, 8);
        while cby < bc {
            if self.inode_use_cache[cby as usize] & (1 << cbi) == 0 {
                self.inode_use_cache[cby as usize] |= 1 << cbi;
                return Some((cby << 3) as u32 + cbi as u32);
            }
            if cbi == 7 {
                cbi = 0;
                cby += 1;
            } else {
                cbi += 1;
            }
        }
        None
    }
    fn alloc_data(&mut self, blocks: u64) -> Option<u64> {
        let mut byte_start: u64 = 0;
        let mut bit_start: u8 = 0;
        let mut byte_end: u64 = 0;
        let mut bit_end: u8 = 0;
        let mut cached_dist: u64 = 0;
        let bc: u64 = crate::common::fastceildiv(self.sup.block_count, 8);
        while byte_end < bc {
            if self.data_use_table[byte_end as usize] & (1 << bit_end) == 0 {
                // this data is unused
                cached_dist += 1;
                if cached_dist == blocks {
                    if byte_end - byte_start > 1 {
                        for n in byte_start + 1..byte_end {
                            self.data_use_table[n as usize] |= 0xff;
                        }
                    }
                    for n in bit_start..8 {
                        self.data_use_table[byte_start as usize] |= 1 << n;
                    }
                    for n in 0..bit_end + 1 {
                        self.data_use_table[byte_end as usize] |= 1 << n;
                    }
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
        let mut res = Self {
            sup: Superblock {
                magic: 0x1815f05f7470ff65,
                data_size: 4194304,
                inode_count: 256,
                data_block_size: 4096,
                block_count: 1024,
                version: FileSystemVersion::V1,
            },
            inode_use_cache: Box::new([0; 32]),
            inodes: Box::new(
                [Inode {
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
                    created: 0,
                }; 256],
            ),
            data_use_table: Box::new([0; 128]),
            data: Box::new([0; 4194304]),
        };
        // need to make . dentry for root
        // TODO: is this block really necessary?
        // real fs init would also make .. TODO
        {
            let i = &mut res.inodes[1];
            i.num = 1;
            // first_block, end_block already default to 0
            i.total_file_size = 4096;
            i.perms = 0o755 | (0x1 << 12);
            i.hard_link_count = 1;
        }
        res.inode_use_cache[0] = 0x2;
        res.data_use_table[0] = 0x1;
        let mut d = Dentry::from_internal(1, &res).unwrap();
        let mut mv = [0u8; 252];
        mv[0] = '.' as u8;
        d.intern.push(DentryEntry {
            inum: 1,
            filename_cstr: mv,
        });
        d.write_back(&mut res);
        res
    }
}

// TODO: much better error handling
impl vfs::VirtualFileSystem for FileSystem {
    fn get_fd(&self, inode: u32, _fd: u32) -> Option<Box<dyn vfs::VirtualFileDescriptor>> {
        if !self.check_inode(inode) {
            return None;
        }
        // valid inode found, we can make the fd!
        Some(Box::new(FileDescriptor {
            inum: inode,
            pos: 0,
        }))
    }
    // note: file deletion might need to be in the context of the dentry
    // since you can't delete a file unless you can stat it
    fn delete_file(&mut self, inode: u32, dir_inode: u32) -> vfs::VfsResult {
        if !self.check_inode(inode) || !self.check_inode(dir_inode) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        let ino = &self.inodes[inode as usize];
        self.clear_data(ino.first_block, ino.end_block).unwrap();
        self.clear_inode(inode).unwrap();
        let mut dentry = Dentry::from_internal(dir_inode, &self).unwrap();
        for n in 0..dentry.intern.len() {
            if dentry.intern[n].inum == inode {
                dentry.intern.remove(n);
                break;
            }
        }
        dentry.write_back(self);
        Ok(())
    }
    // todo: explicit typing
    fn create_file(&mut self, dir_inode: u32, filename: String, data: &[u8]) -> Option<u32> {
        let mut dentry = Dentry::from_internal(dir_inode, &self).unwrap();
        // uniqueness check
        for e in &dentry.intern {
            if e.inum == 0 {
                continue;
            }
            if crate::common::bytes_to_string(&e.filename_cstr) == filename {
                return None;
            }
        }
        let _file_inode = self.alloc_inode();
        if _file_inode.is_none() {
            return None;
        }
        let file_inode: usize = _file_inode.unwrap() as usize;
        let bc = crate::common::fastceildiv(data.len() as u64, self.sup.data_block_size as u64);
        let _first_block = self.alloc_data(bc);
        if _first_block.is_none() {
            self.clear_inode(file_inode as u32).unwrap();
            return None;
        }
        // TODO: factor out self.inodes[file_inode]
        let fb = _first_block.unwrap();
        let sp: usize = (fb * (self.sup.data_block_size as u64)) as usize;
        self.data[sp..sp + data.len()].copy_from_slice(data);
        self.inodes[file_inode].num = file_inode as u32;
        self.inodes[file_inode].first_block = fb;
        self.inodes[file_inode].end_block = fb + bc - 1;
        self.inodes[file_inode].total_file_size = data.len() as u64;
        self.inodes[file_inode].perms = 0o666 & !crate::sysvars::UMASK.load(Ordering::Relaxed);
        // for now we're presuming the user is root
        // TODO: update vfs to allow fs to set/check uid, gid, perms
        self.inodes[file_inode].uid = 0;
        self.inodes[file_inode].gid = 0;
        self.inodes[file_inode].hard_link_count = 1;
        // until we get proper date support, we're just gonna set everything here to 0
        // TODO: actually implement these dates
        self.inodes[file_inode].accessed = 0;
        self.inodes[file_inode].modified = 0;
        self.inodes[file_inode].created = 0;
        let mut mv: [u8; 252] = [0; 252];
        let l = std::cmp::min(252, filename.len());
        mv[..l].copy_from_slice(&filename.as_bytes()[..l]);
        dentry.intern.push(DentryEntry {
            inum: file_inode as u32,
            filename_cstr: mv,
        });
        dentry.write_back(self);
        Some(file_inode as u32)
    }
    // we don't do anything special with fd's in INFS, so these are simple operations
    // however, other fs'es might cache information about where on disk to look (LBA-optimization)
    // that doesn't apply for this implementation though, but we need this flexibility
    fn rewind_zero(&mut self, fd: &mut Box<dyn vfs::VirtualFileDescriptor>) -> vfs::VfsResult {
        if !self.check_inode(fd.get_inum()) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        // position 0 is always valid
        fd.set_pos_raw(0)
    }
    fn rewind(
        &mut self,
        fd: &mut Box<dyn vfs::VirtualFileDescriptor>,
        count: u64,
    ) -> vfs::VfsResult {
        if !self.check_inode(fd.get_inum()) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        let cp: u64 = fd.get_pos();
        if count > cp {
            return Err(vfs::VfsErrno::EFPOOB);
        }
        fd.set_pos_raw(count - cp)
    }
    fn seek_forward(
        &mut self,
        fd: &mut Box<dyn vfs::VirtualFileDescriptor>,
        count: u64,
    ) -> vfs::VfsResult {
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
    fn seek(
        &mut self,
        fd: &mut Box<dyn vfs::VirtualFileDescriptor>,
        location: u64,
    ) -> vfs::VfsResult {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        if location >= self.inodes[i as usize].total_file_size {
            return Err(vfs::VfsErrno::EFPOOB);
        }
        fd.set_pos_raw(location)
    }
    fn read_n(
        &mut self,
        fd: &mut Box<dyn vfs::VirtualFileDescriptor>,
        count: u64,
    ) -> Option<Vec<u8>> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        let p: u64 = fd.get_pos();
        if self.inodes[i as usize].total_file_size < p + count {
            return None;
        }
        let a: usize = self.getcpos(i, p);
        fd.set_pos_raw(p + count).unwrap();
        Some(Vec::from(&self.data[a..a + (count as usize)]))
    }
    fn read_to_eof(&mut self, fd: &mut Box<dyn vfs::VirtualFileDescriptor>) -> Option<Vec<u8>> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        self.read_n(fd, self.inodes[i as usize].total_file_size - fd.get_pos())
    }
    // TODO: factor out duplicate code
    fn write_in_place(
        &mut self,
        fd: &mut Box<dyn vfs::VirtualFileDescriptor>,
        buf: &[u8],
    ) -> vfs::VfsResult {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        let p: u64 = fd.get_pos();
        if self.inodes[i as usize].total_file_size <= p + (buf.len() as u64) {
            return Err(vfs::VfsErrno::EFPOOB);
        }
        let a: usize = self.getcpos(i, p);
        self.data[a..a + buf.len()].copy_from_slice(buf);
        Ok(())
    }
    fn overwrite(
        &mut self,
        fd: &mut Box<dyn vfs::VirtualFileDescriptor>,
        buf: &[u8],
    ) -> vfs::VfsResult {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        let bc = crate::common::fastceildiv(buf.len() as u64, self.sup.data_block_size as u64);
        let _fb = self.alloc_data(bc);
        if _fb.is_none() {
            return Err(vfs::VfsErrno::ENSTOR);
        }
        let fb = _fb.unwrap();
        let sp: usize = (fb * self.sup.data_block_size as u64) as usize;
        self.data[sp..sp + buf.len()].copy_from_slice(buf);
        let ino_s = &self.inodes[i as usize];
        self.clear_data(ino_s.first_block, ino_s.end_block).unwrap();
        // upgrade to mut pointer now that we're not self-mutting
        let ino = &mut self.inodes[i as usize];
        ino.first_block = fb;
        ino.end_block = fb + bc - 1;
        ino.total_file_size = buf.len() as u64;
        // TODO: update ino.modified, ino.accessed
        Ok(())
    }
    // INFS is CoW for appends
    fn append(
        &mut self,
        fd: &mut Box<dyn vfs::VirtualFileDescriptor>,
        buf: &[u8],
    ) -> vfs::VfsResult {
        unimplemented!()
    }
    // TODO: factor out self.inodes[i as usize]
    fn vfd_as_dentry(
        &mut self,
        fd: &Box<dyn vfs::VirtualFileDescriptor>,
    ) -> Option<Box<dyn vfs::VirtualDentry>> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        if self.inodes[i as usize].get_dirtype() != DirType::Dir {
            return None;
        }
        let sp = (self.inodes[i as usize].first_block * (self.sup.data_block_size as u64)) as usize;
        Some(Box::new(Dentry::new(
            &self.data[sp..sp + (self.inodes[i as usize].total_file_size as usize)],
            i,
        )))
    }
    fn file_perms(&self, fd: &Box<dyn vfs::VirtualFileDescriptor>) -> Option<u16> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        Some(self.inodes[i as usize].perms)
    }
    fn file_owner(&self, fd: &Box<dyn vfs::VirtualFileDescriptor>) -> Option<u32> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        Some(self.inodes[i as usize].uid)
    }
    fn file_group(&self, fd: &Box<dyn vfs::VirtualFileDescriptor>) -> Option<u32> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        Some(self.inodes[i as usize].gid)
    }
    fn file_size(&self, fd: &Box<dyn vfs::VirtualFileDescriptor>) -> Option<u64> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        Some(self.inodes[i as usize].total_file_size)
    }
    fn file_modified(&self, fd: &Box<dyn vfs::VirtualFileDescriptor>) -> Option<u64> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        Some(self.inodes[i as usize].modified)
    }
    fn file_hardlinks(&self, fd: &Box<dyn vfs::VirtualFileDescriptor>) -> Option<u16> {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return None;
        }
        Some(self.inodes[i as usize].hard_link_count)
    }
}

// file system structure
// inode = 0 does not exist
// if a dentryentry refers to inode 0, that means nothing there
// V1's magic is 0x1815f05f7470ff65
//                 IRIS-OS-NANO--FS
