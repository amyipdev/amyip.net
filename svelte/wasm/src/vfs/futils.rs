// assumes fs is mounted
// TODO: factor out further (read?)
pub fn read_to_end(mut path: String, short: bool) -> Option<Vec<u8>> {
    // prepare destination string
    if path.ends_with("/") {
        path.push('.');
    }
    if path.starts_with("/") {
        let mut m = path.chars();
        m.next();
        path = m.collect::<String>();
    } else {
        // presumes CWD ends in slash
        path = format!("{}{}", crate::sysvars::load_cwd(), path);
    }
    let fsw = crate::vfs::safe_wrap_fdfs(path.to_string());
    let mut vdent = fsw.0.vfd_as_dentry(&fsw.0.get_fd(1, 0).unwrap()).unwrap();
    let mut target: u32;
    'bb: loop {
        // set into vdent?
        let rem: Vec<String> = path.splitn(2, '/').map(|x| x.to_string()).collect();
        if rem.len() == 2 && rem[0] == "" {
            path = rem[1].clone();
            continue 'bb;
        }
        //if rem.len() == 1 {
        // file MUST be in current directory, or DNE
        for ent in vdent.get_entries() {
            if rem[0] == ent.filename {
                // safe because rem.len() in {1,2}
                if rem.len() == 1 {
                    // file is found, report back inode
                    target = ent.inum;
                    break 'bb;
                } else {
                    // subdir found
                    let fd = fsw.0.get_fd(ent.inum, 0).unwrap();
                    vdent = fsw.0.vfd_as_dentry(&fd).unwrap();
                    path = rem[1].clone();
                    continue 'bb;
                }
            }
        }
        // can't go any further
        return None;
    }
    if short {
        return Some(Vec::from(target.to_le_bytes()));
    }
    let mut fd = fsw.0.get_fd(target, 0).unwrap();
    fsw.0.read_to_eof(&mut fd)
}
