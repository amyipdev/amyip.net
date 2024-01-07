use crate::vfs::*;

use once_cell::sync::Lazy;

static DATA_POOL: Lazy<[u8; 4096]> = Lazy::new(|| {
	let mut i = [0u8; 4096];
	i[..5].clone_from_slice(&[1, 0, 0, 0, '.' as u8]);
	i
});

pub struct FileSystem {}
impl VirtualFileSystem for FileSystem {
	fn get_fd(&self, inode: u32, fd: u32) -> Option<Box<dyn VirtualFileDescriptor>> {
		if inode == 1 {Some(Box::new(FileDescriptor {pos: 0}))} else {None}
	}
	fn delete_file(&mut self, inode: u32, dir_inode: u32) -> VfsResult {
		Err(VfsErrno::EINVFD)
	}
	fn create_file(&mut self, dir_inode: u32, filename: String, data: &[u8]) -> Option<u32> {
		None
	}
	fn rewind_zero(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>) -> VfsResult {
		if fd.get_inum() != 1 {
			return Err(VfsErrno::EINVFD);
		}
		fd.set_pos_raw(0);
		Ok(())
	}
	fn rewind(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, count: u64) -> VfsResult {
		if fd.get_inum() != 1 {
			return Err(VfsErrno::EINVFD);
		}
		let p = fd.get_pos();
		if p >= count {
			fd.set_pos_raw(p - count);
			return Ok(());
		}
		Err(VfsErrno::EFPOOB)
	}
	fn seek_forward(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, count: u64) -> VfsResult {
		if fd.get_inum() != 1 {
			return Err(VfsErrno::EINVFD);
		}
		let p = fd.get_pos() + count;
		if p >= 4096 {
			return Err(VfsErrno::EFPOOB);
		}
		fd.set_pos_raw(p);
		Ok(())
	}
	fn seek(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, location: u64) -> VfsResult {
		if fd.get_inum() != 1 {
			return Err(VfsErrno::EINVFD);
		}
		if location < 4096 {
			fd.set_pos_raw(location);
			return Ok(());
		}
		Err(VfsErrno::EFPOOB)
	}
	fn read_n(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, count: u64) -> Option<Vec<u8>> {
		if fd.get_inum() != 1 {
			return None;
		}
		let cp = fd.get_pos() as usize;
		let c = count as usize;
		let d = cp + c;
		if d > 4096 {
			return None;
		}
		fd.set_pos_raw(d as u64);
		Some(DATA_POOL[cp..d].try_into().unwrap())
	}
	fn read_to_eof(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>) -> Option<Vec<u8>> {
		self.read_n(fd, 4096 - fd.get_pos())
	}
	fn write_in_place(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, buf: &[u8]) -> VfsResult {
		Err(VfsErrno::ENSTOR)
	}
	fn overwrite(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, buf: &[u8]) -> VfsResult {
		Err(VfsErrno::ENSTOR)
	}
	fn append(&mut self, fd: &mut Box<dyn VirtualFileDescriptor>, buf: &[u8]) -> VfsResult {
		Err(VfsErrno::ENSTOR)
	}
    fn vfd_as_dentry(
		&mut self,
		fd: &Box<dyn VirtualFileDescriptor>,
		) -> Option<Box<dyn VirtualDentry>> {
		if fd.get_inum() != 1 {
			return None;
		}
		return Some(Box::new(Dentry {}))
	}
	fn file_perms(&self, fd: &Box<dyn VirtualFileDescriptor>) -> Option<u16> {
		Some(0x1000 + 0o440)
	}
	fn file_owner(&self, fd: &Box<dyn VirtualFileDescriptor>) -> Option<u32> {
		Some(0)
	}
	fn file_group(&self, fd: &Box<dyn VirtualFileDescriptor>) -> Option<u32> {
		Some(0)
	}
	fn file_size(&self, fd: &Box<dyn VirtualFileDescriptor>) -> Option<u64> {
		Some(4096)
	}
	fn file_modified(&self, fd: &Box<dyn VirtualFileDescriptor>) -> Option<u64> {
		Some(0)
	}
	fn file_hardlinks(&self, fd: &Box<dyn VirtualFileDescriptor>) -> Option<u16> {
		Some(1)
	}
}

struct FileDescriptor {
	pos: u64
}
impl VirtualFileDescriptor for FileDescriptor {
	fn get_inum(&self) -> u32 {
		1
	}
	fn get_pos(&self) -> u64 {
		self.pos
	}
	fn set_pos_raw(&mut self, pos: u64) -> VfsResult {
		self.pos = pos;
		Ok(())
	}
}

struct Dentry {}
impl VirtualDentry for Dentry {
	fn get_entries(&self) -> Vec<VirtualDentryEntry> {
		vec![VirtualDentryEntry {inum: 1, filename: ".".to_string()}]
	}
	fn get_inode(&self) -> u32 {
		1
	}
}

