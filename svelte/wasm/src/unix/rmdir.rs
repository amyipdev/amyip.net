use crate::errors::ao;
use xterm_js_rs::Terminal;

pub fn rmdir(term: &Terminal, args: Vec<&str>) -> i32 {
    if args.len() < 1 {
        term.writeln("rmdir: missing file operand");
        crate::common::minfo(term, "rmdir");
        return -1;
    }
    if args.len() > 1 {
        term.writeln("rmdir: too many arguments");
        crate::common::minfo(term, "rmdir");
        return -2;
    }
    if args[0] == "--help" {
        term.writeln("Usage: rmdir [target]");
        return 0;
    }
    let mut dd = ao!(
        crate::vfs::futils::find_file(args[0].to_string(), false).left(),
        ah,
        -3,
        term
    );
    if dd.0.file_perms(&dd.1).unwrap() & 0xf000 != 0x1000 {
        term.writeln("rmdir: cannot remove directory: Not a directory");
        return -4;
    }
    recurse_dir(&mut dd.0, &dd.1);
    let pd = ao!(
        crate::vfs::futils::find_file(
            args[0].rsplitn(2, '/').nth(1).unwrap_or(".").to_string(),
            false
        )
        .left(),
        ah,
        -3,
        term
    );
    pd.0.delete_file(dd.1.get_inum(), pd.1.get_inum());
    return 0;
}

// Assumes directory is pre-checked
fn recurse_dir(
    fs: &mut Box<dyn crate::vfs::VirtualFileSystem>,
    dent: &Box<dyn crate::vfs::VirtualFileDescriptor>,
) {
    let mut ents = fs.vfd_as_dentry(dent).unwrap();
    for ent in ents.get_entries() {
        if ent.filename == "." || ent.filename == ".." {
            continue;
        }
        let mut fd = fs.get_fd(ent.inum, 0).unwrap();
        if fs.file_perms(&fd).unwrap() & 0xf000 == 0x1000 {
            recurse_dir(fs, &fd);
        }
        fs.delete_file(ent.inum, ents.get_inode()).unwrap();
    }
}

fn ah(term: &Terminal, code: i32) {
    term.writeln(match code {
        -3 => "rmdir: cannot remove directory: No such file or directory",
        _ => "rmdir: unknown error",
    });
}
