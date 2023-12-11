// the IrisOS-Nano File System (INFS)

use crate::vfs;

struct FileSystem {
    sup: Superblock,
    inode_use_cache: Box<[u8]>,
    inodes: Box<[Inode]>,
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

struct FileDescriptor<'a> {
    // inode number
    inum: u32,
    // position
    pos: u64,
    // fd number
    fd: u32,
    // pointer back to filesystem
    fs: &'a mut FileSystem
}

struct DentryEntry {
    inum: u32,
    filename_cstr: [u8; 252]
}

//impl vfs::VirtualFileSystem for FileSystem {
    
//}

// file system structure
//
// data blocks are numbered 0..n, and on every (x mod data_block_size*8)'th block, a block is stored
//   where every bit corresponds to whether the data blocks in that row are used
// inode = 0 does not exist
// if a dentryentry refers to inode 0, that means nothing there
// V1's magic is 0x1815f05f7470ff65
//                 IRIS-OS-NANO--FS