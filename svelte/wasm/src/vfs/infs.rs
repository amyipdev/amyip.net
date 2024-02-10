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

// TODO: consider having a local-storage mount option
// - allows disk to be persistent
// - less memory hit

use crate::vfs;
use crate::vfs::VirtualFileSystem;
use std::sync::atomic::Ordering;

// Box<[u8]> is cool, but Vec<u8> ends up being necessary
// because on fs creation, [u8] is unsized
pub struct FileSystem {
    sup: Superblock,
    inode_use_cache: Vec<u8>,
    pub inodes: Vec<Inode>,
    data_use_table: Vec<u8>,
    // since WASM is in-memory, we can afford to do this
    data: Vec<u8>,
}

enum FileSystemVersion {
    V1,
}
impl FileSystemVersion {
    pub fn from(inp: u64) -> Option<Self> {
        match inp {
            0x1815f05f7470ff65 => Some(Self::V1),
            _ => None,
        }
    }
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
pub fn get_sup(inodes: u32, block_size: u32, num_blocks: u64) -> Superblock {
    Superblock {
        magic: 0x1815f05f7470ff65,
        data_size: (block_size as u64) * num_blocks,
        inode_count: inodes,
        data_block_size: block_size,
        block_count: num_blocks,
        version: FileSystemVersion::V1,
    }
}
// TODO: deprecate create_test_fs
// TODO: put into impl FileSystem
pub fn mknrfs(inodes: u32, block_size: u32, num_blocks: u64) -> FileSystem {
    let d = (block_size as u64) * num_blocks;
    let mut r = FileSystem {
        sup: vfs::infs::get_sup(inodes, block_size, num_blocks),
        inode_use_cache: vec![0; (inodes as usize) >> 3],
        inodes: vec![
            Inode {
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
            };
            inodes as usize
        ],
        data_use_table: vec![0; (num_blocks >> 3) as usize],
        data: vec![0; d as usize],
    };
    {
        let i = &mut r.inodes[1];
        i.num = 1;
        // first_block, end_block already default to 0
        i.total_file_size = 4096;
        i.perms = 0o10755;
        i.hard_link_count = 1;
    }
    r.inode_use_cache[0] = 0x2;
    r.data_use_table[0] = 0x1;
    let mut d = Dentry::from_internal(1, &r).unwrap();
    let mut p1 = [0u8; 252];
    let mut p2 = [0u8; 252];
    p1[0] = '.' as u8;
    p2[0] = '.' as u8;
    p2[1] = '.' as u8;
    d.intern.push(DentryEntry {
        inum: 1,
        filename_cstr: p1,
    });
    d.intern.push(DentryEntry {
        inum: 1,
        filename_cstr: p2,
    });
    d.write_back(&mut r, true);
    r
}

// inode structure length is 64
// TODO: consider implementing Default
#[derive(Copy, Clone, Debug)]
pub struct Inode {
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
    fn write_back(self, fs: &mut FileSystem, first: bool) {
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
        if !first {
            fs.overwrite(&mut fs.get_fd(self.inum, 0x0).unwrap(), &ba)
                .unwrap();
        } else {
            // when creating a directory, we don't want to just delete what's at the position
            // overwrite calls clear_data - but our data blocks probably aren't right
            // this is a modified version of overwrite without this issue
            let bc = crate::common::fastceildiv(ba.len() as u64, fs.sup.data_block_size as u64);
            let _fb = fs.alloc_data(bc);
            let fb = _fb.unwrap();
            let sp: usize = (fb * fs.sup.data_block_size as u64) as usize;
            fs.data[sp..sp + ba.len()].copy_from_slice(&ba);
            let ino = &mut fs.inodes[self.inum as usize];
            ino.first_block = fb;
            ino.end_block = fb + bc - 1;
            ino.total_file_size = ba.len() as u64;
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
        self.inodes[inode as usize] = Inode {
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
        };
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
        if sby != eby {
            for n in sbi..8 {
                self.data_use_table[sby as usize] &= !(1 << n);
            }
            for n in 0..ebi + 1 {
                self.data_use_table[eby as usize] &= !(1 << n);
            }
        } else {
            for n in sbi..ebi + 1 {
                self.data_use_table[eby as usize] &= !(1 << n);
            }
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
                    if byte_start != byte_end {
                        for n in bit_start..8 {
                            self.data_use_table[byte_start as usize] |= 1 << n;
                        }
                        for n in 0..bit_end + 1 {
                            self.data_use_table[byte_end as usize] |= 1 << n;
                        }
                    } else {
                        for n in bit_start..bit_end + 1 {
                            self.data_use_table[byte_end as usize] |= 1 << n;
                        }
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
            inode_use_cache: vec![0; 32],
            // TODO: change into vec![] syntax
            inodes: Vec::from(
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
            data_use_table: vec![0; 128],
            data: vec![0; 4194304],
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
        d.write_back(&mut res, true);
        res
    }
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 32 {
            return None;
        }
        let mag: u64 = u64::from_le_bytes(buf[..8].try_into().unwrap());
        let sup: Superblock = Superblock {
            magic: mag,
            data_size: u64::from_le_bytes(buf[8..16].try_into().unwrap()),
            inode_count: u32::from_le_bytes(buf[16..20].try_into().unwrap()),
            data_block_size: u32::from_le_bytes(buf[20..24].try_into().unwrap()),
            block_count: u64::from_le_bytes(buf[24..32].try_into().unwrap()),
            version: FileSystemVersion::from(mag).expect(&format!("{:#x}", mag)),
        };
        if sup.inode_count % 8 != 0 || sup.data_block_size % 256 != 0 {
            return None;
        }
        let start_inode_table: usize = 32;
        let start_inodes: usize = start_inode_table + ((sup.inode_count as usize) >> 3);
        let start_data_table: usize = start_inodes + ((sup.inode_count as usize) << 6);
        let start_data: usize = start_data_table + ((sup.block_count as usize) >> 3);
        let end_fs: usize = start_data + sup.block_count as usize * sup.data_block_size as usize;
        if buf.len() != end_fs {
            return None;
        }
        Some(Self {
            sup: sup,
            inode_use_cache: buf[start_inode_table..start_inodes].try_into().unwrap(),
            inodes: {
                let dat = &buf[start_inodes..start_data_table];
                let mut res = vec![];
                for n in 0..dat.len() >> 6 {
                    let bp = n << 6;
                    res.push(Inode {
                        num: u32::from_le_bytes(dat[bp..bp + 4].try_into().unwrap()),
                        first_block: u64::from_le_bytes(dat[bp + 4..bp + 12].try_into().unwrap()),
                        end_block: u64::from_le_bytes(dat[bp + 12..bp + 20].try_into().unwrap()),
                        total_file_size: u64::from_le_bytes(
                            dat[bp + 20..bp + 28].try_into().unwrap(),
                        ),
                        perms: u16::from_le_bytes(dat[bp + 28..bp + 30].try_into().unwrap()),
                        uid: u32::from_le_bytes(dat[bp + 30..bp + 34].try_into().unwrap()),
                        gid: u32::from_le_bytes(dat[bp + 34..bp + 38].try_into().unwrap()),
                        hard_link_count: u16::from_le_bytes(
                            dat[bp + 38..bp + 40].try_into().unwrap(),
                        ),
                        accessed: u64::from_le_bytes(dat[bp + 40..bp + 48].try_into().unwrap()),
                        modified: u64::from_le_bytes(dat[bp + 48..bp + 56].try_into().unwrap()),
                        created: u64::from_le_bytes(dat[bp + 56..bp + 64].try_into().unwrap()),
                    })
                }
                res
            },
            data_use_table: buf[start_data_table..start_data].try_into().unwrap(),
            data: buf[start_data..end_fs].try_into().unwrap(),
        })
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![];
        res.extend(self.sup.magic.to_le_bytes());
        res.extend(self.sup.data_size.to_le_bytes());
        res.extend(self.sup.inode_count.to_le_bytes());
        res.extend(self.sup.data_block_size.to_le_bytes());
        res.extend(self.sup.block_count.to_le_bytes());
        res.extend(self.inode_use_cache.clone());
        for n in &self.inodes {
            res.extend(n.num.to_le_bytes());
            res.extend(n.first_block.to_le_bytes());
            res.extend(n.end_block.to_le_bytes());
            res.extend(n.total_file_size.to_le_bytes());
            res.extend(n.perms.to_le_bytes());
            res.extend(n.uid.to_le_bytes());
            res.extend(n.gid.to_le_bytes());
            res.extend(n.hard_link_count.to_le_bytes());
            res.extend(n.accessed.to_le_bytes());
            res.extend(n.modified.to_le_bytes());
            res.extend(n.created.to_le_bytes());
        }
        res.extend(self.data_use_table.clone());
        res.extend(self.data.clone());
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
    // TODO: check to make sure hardlinks = 1; if not, not safe to delete, just remove from dentry
    // TODO:   (should decrement hardlinks)
    fn delete_file(&mut self, inode: u32, dir_inode: u32) -> vfs::VfsResult {
        if !self.check_inode(inode) || !self.check_inode(dir_inode) || inode == dir_inode || inode == 0 {
            return Err(vfs::VfsErrno::EINVFD);
        }
        self.inodes[inode as usize].hard_link_count -= 1;
        let ino = &self.inodes[inode as usize];
        if ino.hard_link_count == 0 {
            self.clear_data(ino.first_block, ino.end_block).unwrap();
            self.clear_inode(inode).unwrap();
        }
        let mut dentry = Dentry::from_internal(dir_inode, &self).unwrap();
        for n in 0..dentry.intern.len() {
            if dentry.intern[n].inum == inode {
                let fx = crate::common::bytes_to_string(&dentry.intern[n].filename_cstr);
                if fx == "." || fx == ".." {
                    return Err(vfs::VfsErrno::EINVFD);
                }
                dentry.intern.remove(n);
                break;
            }
        }
        dentry.write_back(self, false);
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
        if _first_block.is_none() && bc != 0 {
            self.clear_inode(file_inode as u32).unwrap();
            return None;
        }
        // TODO: factor out self.inodes[file_inode]
        let fb = _first_block.unwrap_or(1);
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
        dentry.write_back(self, false);
        Some(file_inode as u32)
    }
    fn create_directory(&mut self, parent_inode: u32, name: String) -> Option<u32> {
        if !self.check_inode(parent_inode) {
            return None;
        }
        let _nino = self.alloc_inode();
        if _nino.is_none() {
            return None;
        }
        let nino: usize = _nino.unwrap() as usize;
        self.inodes[nino].num = nino as u32;
        let mut pdent = Dentry::from_internal(parent_inode, &self).unwrap();
        self.inodes[nino].uid = 0;
        self.inodes[nino].gid = 0;
        self.inodes[nino].perms = 0o10777 & !crate::sysvars::UMASK.load(Ordering::Relaxed);
        self.inodes[nino].hard_link_count = 1;
        self.inodes[nino].accessed = 0;
        self.inodes[nino].modified = 0;
        self.inodes[nino].created = 0;
        let mut qdent = Dentry::from_internal(nino as u32, &self).unwrap();
        let mut pm1: [u8; 252] = [0; 252];
        let l0 = std::cmp::min(252, name.len());
        pm1[..l0].copy_from_slice(&name.as_bytes()[..l0]);
        pdent.intern.push(DentryEntry {
            inum: nino as u32,
            filename_cstr: pm1,
        });
        let mut qm1: [u8; 252] = [0; 252];
        let mut qm2: [u8; 252] = [0; 252];
        qm1[0] = '.' as u8;
        qm2[0] = '.' as u8;
        qm2[1] = '.' as u8;
        qdent.intern.push(DentryEntry {
            inum: nino as u32,
            filename_cstr: qm1,
        });
        qdent.intern.push(DentryEntry {
            inum: parent_inode,
            filename_cstr: qm2,
        });
        qdent.write_back(self, true);
        pdent.write_back(self, false);
        Some(nino as u32)
    }
    fn hardlink(
        &mut self,
        parent_inode: u32,
        deploy_inode: u32,
        name: String,
    ) -> crate::vfs::VfsResult {
        if !self.check_inode(parent_inode) || !self.check_inode(deploy_inode) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        let mut p = Dentry::from_internal(parent_inode, &self).unwrap();
        let mut mv: [u8; 252] = [0; 252];
        let l = std::cmp::min(252, name.len());
        mv[..l].copy_from_slice(&name.as_bytes()[..l]);
        p.intern.push(DentryEntry {
            inum: deploy_inode,
            filename_cstr: mv,
        });
        self.inodes[deploy_inode as usize].hard_link_count += 1;
        p.write_back(self, false);
        Ok(())
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
        // CRITICAL TODO: implement this
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
    fn chmod(&mut self, fd: &Box<dyn vfs::VirtualFileDescriptor>, perms: u16) -> vfs::VfsResult {
        let i: u32 = fd.get_inum();
        if !self.check_inode(i) {
            return Err(vfs::VfsErrno::EINVFD);
        }
        self.inodes[i as usize].perms = perms;
        Ok(())
    }
}

// file system structure
// inode = 0 does not exist
// if a dentryentry refers to inode 0, that means nothing there
// V1's magic is 0x1815f05f7470ff65
//                 IRIS-OS-NANO--FS
