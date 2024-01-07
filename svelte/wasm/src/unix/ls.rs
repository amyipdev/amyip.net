use xterm_js_rs::Terminal;

const LS_HELP: &str = "Usage: ls [OPTION]... [FILE]...
List information about the FILEs (the current directory by default).
Entries are sorted alphabetically.


 -a, --all             do not ignore entries starting with .
 -A, --almost-all      do not list implied . and ..
 -h, --human-readable  with -l, print sizes like 1K 234M 2G etc.
     --si              likewise, but use powers of 1000 not 1024
 -i, --inode           print the index number of each file
 -l                    use a long listing format
 --help                display this help and exit
 --version             output version information and exit 

Many POSIX ls features are not supported; see ls --version for more info.";
const LS_VSN: &str = "ls (IrisOS-nano) 0.1
Copyright (C) Amy Parker, 2023
License AGPLv3+: GNU AGPL version 3 or later <https://gnu.org/licenses/agpl.html>
This is free software; you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Amy Parker <amy@amyip.net>.
Based on ls by Richard M. Stallman and David MacKenzie.";
struct LsOpts {
    all: bool,
    almost_all: bool,
    human_readable: bool,
    si: bool,
    inode: bool,
    longlist: bool,
    help: bool,
    version: bool,
}
// TODO: check if /etc/passwd, /etc/group exists, and if so, print user/group name
pub fn ls(term: &Terminal, mut args: Vec<&str>) -> i32 {
    let mut opt = LsOpts {
        all: false,
        almost_all: false,
        human_readable: false,
        si: false,
        inode: false,
        longlist: false,
        help: false,
        version: false,
    };
    // TODO: dedup (POSIX tools process largely the same)
    let mut split = args.len();
    for n in 0..args.len() {
        if !args[n].starts_with('-') {
            split = n;
            break;
        }
    }
    // TODO: does this (as well as in cat) need to be mut?
    let (opts, files) = args.split_at_mut(split);
    for arg in opts {
        if arg.chars().take(2).last().unwrap() == '-' {
            match *arg {
                "--help" => opt.help = true,
                "--version" => opt.version = true,
                "--all" => opt.all = true,
                "--almost-all" => opt.almost_all = true,
                "--human-readable" => opt.human_readable = true,
                "--inode" => opt.inode = true,
                "--si" => opt.si = true,
                _ => {
                    term.writeln(&format!("ls: unrecognized option '{}'", arg));
                    crate::common::minfo(term, "ls");
                    return 1;
                }
            }
            continue;
        }
        for n in arg.get(1..).unwrap().chars() {
            match n {
                'a' => opt.all = true,
                'A' => opt.almost_all = true,
                'h' => opt.human_readable = true,
                'i' => opt.inode = true,
                'l' => opt.longlist = true,
                _ => {
                    term.writeln(&format!("ls: invalid option -- '{}'", n));
                    crate::common::minfo(term, "ls");
                    return 2;
                }
            }
        }
    }
    if opt.help {
        term.writeln(LS_HELP);
        return 0;
    }
    if opt.version {
        term.writeln(LS_VSN);
        return 0;
    }
    let flen = files.len();
    if flen == 0 {
        // check .
        process_dir(term, ".", &opt);
    }
    for f in files {
        if flen != 1 {
            term.writeln(&format!("{}:", f));
        }
        process_dir(term, f, &opt);
        if flen != 1 {
            term.writeln("");
        }
    }
    return 0;
}
struct FileEntry {
    perms: u16,
    owner: u32,
    group: u32,
    tfs: u64,
    modified: u64,
    filename: String,
    inode: u32,
    hardlinks: u16,
}
fn process_dir(term: &Terminal, dir: &str, opt: &LsOpts) {
    let mut files: Vec<FileEntry> = vec![];
    // we still need to get the FS to read, but we can
    // abuse read_to_end's short-circuit operation
    let ino: u32 = u32::from_le_bytes(
        crate::vfs::futils::read_to_end(dir.to_string(), true)
            .unwrap_or(u32::MAX.to_le_bytes().try_into().unwrap())
            .try_into()
            .unwrap(),
    );
    if ino == u32::MAX {
        term.writeln(&format!("ls: {}: No such file or directory", dir));
        return;
    }
    let fsw = crate::vfs::safe_wrap_fdfs(dir.to_string()).0;
    let fd = fsw.get_fd(ino, 0).unwrap();
    if fsw.file_perms(&fd).unwrap() >> 12 != 0x1 {
        files.push(FileEntry {
            perms: fsw.file_perms(&fd).unwrap(),
            owner: fsw.file_owner(&fd).unwrap(),
            group: fsw.file_group(&fd).unwrap(),
            tfs: fsw.file_size(&fd).unwrap(),
            modified: fsw.file_modified(&fd).unwrap(),
            filename: dir.to_string(),
            inode: ino,
            hardlinks: fsw.file_hardlinks(&fd).unwrap(),
        });
    } else {
        let vdent = fsw.vfd_as_dentry(&fd).unwrap();
        for e in vdent.get_entries() {
            if e.filename.starts_with('.')
                && (!(opt.all || opt.almost_all)
                    || (opt.almost_all && (e.filename == "." || e.filename == "..")))
            {
                continue;
            }
            let nfd = fsw.get_fd(e.inum, 0).unwrap();
            files.push(FileEntry {
                perms: fsw.file_perms(&nfd).unwrap(),
                owner: fsw.file_owner(&nfd).unwrap(),
                group: fsw.file_group(&nfd).unwrap(),
                tfs: fsw.file_size(&nfd).unwrap(),
                modified: fsw.file_modified(&nfd).unwrap(),
                filename: e.filename,
                inode: e.inum,
                hardlinks: fsw.file_hardlinks(&fd).unwrap(),
            });
        }
    }
    // good vector instructions in wasm would really help...
    // TODO: evaluate if u64 is really necessary for the longest_ vars
    let mut longest_inum: u64 = 0;
    for f in &files {
        longest_inum = std::cmp::max(longest_inum, (f.inode.ilog10() + 1) as u64);
    }
    if opt.longlist {
        let mut total: u64 = 0;
        let mut longest_uid: u64 = 1;
        let mut longest_gid: u64 = 1;
        let mut longest_size: u64 = 1;
        let mut longest_hlc: u64 = 1;
        // two passes: gather data, then write
        for f in &files {
            if f.perms >> 12 == 0 {
                total += f.tfs;
            }
            longest_uid = std::cmp::max(longest_uid, if f.owner != 0 {(f.owner.ilog10() + 1) as u64} else {0});
            longest_gid = std::cmp::max(longest_gid, if f.group != 0 {(f.group.ilog10() + 1) as u64} else {0});
            longest_size = std::cmp::max(
                longest_size,
                calculate_size_chars_necessary(f.tfs, opt.human_readable, opt.si),
            );
            longest_hlc = std::cmp::max(longest_hlc, if f.hardlinks != 0 {(f.hardlinks.ilog10() + 1) as u64} else {0});
        }
        term.writeln(&format!("total {}", total));
        for f in files {
            if opt.inode {
                term.write(&format!(
                    "{} ",
                    crate::common::shift_in_text(&f.inode.to_string(), longest_inum as usize)
                ));
            }
            term.write(if f.perms & (1 << 11) != 0 {
                "a"
            } else {
                match f.perms >> 12 {
                    0x0 => "-",
                    0x1 => "d",
                    0x2 => "l",
                    _ => panic!("unsupported file type"),
                }
            });
            term.write(if f.perms & (1 << 8) != 0 { "r" } else { "-" });
            term.write(if f.perms & (1 << 7) != 0 { "w" } else { "-" });
            term.write(if f.perms & (1 << 10) != 0 {
                "s"
            } else {
                if f.perms & (1 << 8) != 0 {
                    "x"
                } else {
                    "-"
                }
            });
            term.write(if f.perms & (1 << 5) != 0 { "r" } else { "-" });
            term.write(if f.perms & (1 << 4) != 0 { "w" } else { "-" });
            term.write(if f.perms & (1 << 9) != 0 {
                "s"
            } else {
                if f.perms & (1 << 8) != 0 {
                    "x"
                } else {
                    "-"
                }
            });
            term.write(if f.perms & (1 << 2) != 0 { "r" } else { "-" });
            term.write(if f.perms & (1 << 1) != 0 { "w" } else { "-" });
            term.write(if f.perms & (1 << 0) != 0 { "x" } else { "-" });
            // TODO: add color to file name
            term.writeln(&format!(
                " {} {} {}  {} {} {}",
                crate::common::shift_in_text(&f.hardlinks.to_string(), longest_hlc as usize),
                crate::common::shift_in_text(&f.owner.to_string(), longest_uid as usize),
                crate::common::shift_in_text(&f.group.to_string(), longest_gid as usize),
                crate::common::shift_in_text(
                    &format_sizes(f.tfs, opt.human_readable, opt.si),
                    longest_size as usize
                ),
                to_datetime(f.modified),
                f.filename,
            ));
        }
    } else {
        // We could support columns, but it's unnecessarily intensive
        // Instead, we just print out line by line
        // TODO: support columns
        for f in files {
            if opt.inode {
                term.write(&format!(
                    "{} ",
                    crate::common::shift_in_text(&f.inode.to_string(), longest_inum as usize)
                ));
            }
            term.writeln(&f.filename);
        }
    }
}
fn to_datetime(unix_timestamp: u64) -> String {
    "Jan  1 00:00".to_string()
}
// TODO: evaluate move to common
fn calculate_size_chars_necessary(num: u64, human: bool, si: bool) -> u64 {
    if human && ((!si && num < 1024) || (si && num < 1000)) {
        if !si {
            (num / 1024u64.pow(num.ilog(1024)) as u64).ilog10() as u64 + 1
        } else {
            let x = (num.ilog10() + 1) % 3;
            if x == 0 {
                3
            } else {
                x as u64
            }
        }
    } else {
        num.ilog10() as u64 + 1
    }
}
// TODO: move out to common
fn format_sizes(num: u64, human: bool, si: bool) -> String {
    let div = if si { 1000 } else { 1024 };
    if !human || num < div {
        return num.to_string();
    }
    let log = num.ilog(div);
    let size: char = match log {
        1 => 'K',
        2 => 'M',
        3 => 'G',
        4 => 'T',
        5 => 'P',
        6 => 'E',
        7 => 'Z',
        8 => 'Y',
        _ => panic!("unsupported sizer"),
    };
    format!("{}{}", num / div.pow(log), size)
}
