use xterm_js_rs::Terminal;

// avoid options - just recurse automatically
pub fn cp(term: &Terminal, args: Vec<&str>) -> i32 {
	if args.len() == 1 && args[0] == "--help" {
		term.writeln("Usage: cp [src] [dest]");
		return 0;
	}
	if args.len() < 2 {
		term.writeln("cp: missing file operand");
		crate::common::minfo(term, "cp");
		return -1;
	}
	if args.len() > 2 {
		term.writeln("cp: too many arguments");
		crate::common::minfo(term, "cp");
		return -1;
	}
	let mut src = crate::vfs::futils::find_file(args[0].to_string(), false).left().unwrap();
	// TODO: optimize out this double rsplitn call
	let dds = args[1].rsplitn(2, '/').nth(1).unwrap_or(".").to_string();
	let f = args[1].rsplitn(2, '/').nth(0).unwrap().to_string();
	let perms = src.0.file_perms(&src.1).unwrap();
	let destdir = crate::vfs::futils::find_file(dds, false)
			.left()
			.unwrap();
	let dd = destdir.1.get_inum();
	if src.0.file_perms(&src.1).unwrap() & 0xf000 == 0x1000 {
		let dino = destdir.0.create_directory(dd, f).unwrap();
		destdir.0.chmod(&destdir.0.get_fd(dino, 0).unwrap(), perms);
		recurse_dir(args[0].to_string(), args[1].to_string());
	} else {
		let fc = src.0.read_to_eof(&mut src.1).unwrap();
		let fx = destdir.0.create_file(dd, f, &fc).unwrap();
		destdir.0.chmod(&destdir.0.get_fd(fx, 0).unwrap(), perms);
	}
	return 0;
}

// inspired from infsprogs
// assumes src and dest are directories
// TODO: this may write onto the wrong FS. dest dir should not just pull the inode,
// TODO: but should actually be a comprehensive pull, and write operations should be
// TODO: on dd.0, not ds.0. this will cause problems...
fn recurse_dir(src: String, dest: String) {
	let mut sd = crate::vfs::futils::find_file(src.clone(), false)
		.left()
		.unwrap();
	let dx = crate::vfs::futils::find_file(dest.clone(), false)
		.left()
		.unwrap();
	let dd = dx.1.get_inum();
	let sdent = sd.0.vfd_as_dentry(&sd.1).unwrap();
	for f in sdent.get_entries() {
		if f.filename == "." || f.filename == ".." {
			continue;
		}
		let ino = f.inum;
		let mut fd = sd.0.get_fd(ino, 0).unwrap();
		// TODO: support copying owner (requires chown in VFS)
		let perms = sd.0.file_perms(&fd).unwrap();
		if perms & 0xf000 == 0x1000 {
			// dentry copy
			let ds = sd.0.create_directory(dd, f.filename.clone()).unwrap();
			sd.0.chmod(&sd.0.get_fd(ds, 0).unwrap(), perms);
			let mut dstr = dest.clone();
			dstr.push('/');
			dstr.push_str(&f.filename);
			let mut sstr = src.clone();
			sstr.push('/');
			sstr.push_str(&f.filename);
			recurse_dir(sstr, dstr);
		} else {
			// just copy the files
			// TODO: dedup with the main function
			let fc = dx.0.read_to_eof(&mut fd).unwrap();
			let fx = dx.0.create_file(dd, f.filename, &fc).unwrap();
			dx.0.chmod(&dx.0.get_fd(fx, 0).unwrap(), perms);
		}
	}
}