// The functions in this module are specific
// to IrisOS-nano. They should never be implemented
// on full releases of IrisOS, or any non-experimental OS
// for that matter.

use crate::vfs::VirtualFileSystem;
use colored::Colorize;
use once_cell::sync::Lazy;
use xterm_js_rs::Terminal;

pub fn kmsg(term: &Terminal, args: Vec<&str>) -> i32 {
    crate::kmessage(term, &args.join(" "));
    return 0;
}

// This one could have a similar command on full Iris,
// but definitely not this particular implementation.
static INFO_MSG: Lazy<String> = Lazy::new(|| {
    format!(
        "{} v{} @ {}
Built in {} by {} {}
Licensed under AGPLv3 {}

IrisOS-nano is a simple {} web operating system,
hosted right here on {}. There are two main goals for it:
1. Be able to have all the same functionality as the rest of
   amyip.net, but in a command-line format, and
2. Serve as a test platform for future apps targeting IrisOS,
   a {} Unix-like OS.
As such, this also serves as a copy of my resume, and more.

Source code: {}",
        "IrisOS-nano".bold().bright_green(),
        env!("CARGO_PKG_VERSION").bold(),
        "amyip.net".bright_cyan(),
        "Rust".bright_yellow(),
        "Amy Parker".bold(),
        "<amy@amyip.net>".bright_black(),
        "<https://gnu.org/licenses/agpl.html>".bright_black(),
        "Unix-like".bold(),
        "amyip.net".bright_cyan(),
        "future-development".bright_red(),
        "<https://github.com/amyipdev/amyip.net>".bright_black()
    )
});
pub fn iris_info(term: &Terminal, _args: Vec<&str>) -> i32 {
    term.writeln(&INFO_MSG);
    return 0;
}

// TODO: eventualy delete once fs stable
pub fn test_infs(term: &Terminal, args: Vec<&str>) -> i32 {
    if args.len() == 0 {
        term.writeln("test-infs: cannot test with no args");
        return 1;
    }
    // no need to build a mountable fs, just run all the tests here
    let mut fs = crate::vfs::infs::FileSystem::create_test_fs();
    term.writeln("created fs");
    // we know the root dentry is at 1
    let ino: u32 = fs
        .create_file(1, "test.txt".to_string(), args[0].as_bytes())
        .unwrap();
    term.writeln("wrote file test.txt");
    // this fd isn't getting stored, so fd number doesn't matter on INFS
    let mut fd = fs.get_fd(ino, 0).unwrap();
    term.write("read to eof on test.txt: ");
    term.writeln(&String::from_utf8(fs.read_to_eof(&mut fd).unwrap()).unwrap());
    if args.len() >= 2 {
        term.writeln("testing multi-file support");
        let mut inos: Vec<u32> = vec![];
        for n in 1..args.len() {
            inos.push(
                fs.create_file(1, format!("test{}.txt", n), args[n].as_bytes())
                    .unwrap(),
            );
            term.writeln(&format!("wrote file test{}.txt", n));
        }
        let mut fds = vec![];
        for n in inos {
            fds.push(fs.get_fd(n, 0).unwrap());
        }
        for mut n in fds {
            term.write("read to eof on multi: ");
            term.writeln(&String::from_utf8(fs.read_to_eof(&mut n).unwrap()).unwrap());
            fs.delete_file(n.get_inum(), 1).unwrap();
            term.writeln("deleted multi");
        }
    }
    term.writeln("deleting file test.txt");
    fs.delete_file(ino, 1).unwrap();
    term.writeln("successfully deleted test.txt");
    term.writeln("INFS driver works correctly");

    let ino = fs
        .create_file(1, "mod.txt".to_string(), "FS traversal worked!".as_bytes())
        .unwrap();
    term.writeln("created file mod.txt");
    crate::vfs::mount_root(Box::new(fs));
    term.writeln("mounted as rootfs");
    let tun = crate::vfs::safe_wrap_fdfs("mod.txt".to_string());
    // TODO: dentry searching and other dentry ops
    let mut fd = tun.0.get_fd(ino, 0).unwrap();
    term.writeln(&String::from_utf8(tun.0.read_to_eof(&mut fd).unwrap()).unwrap());

    return 0;
}

// TODO: eventually delete once fs stable
pub fn test_read_root(term: &Terminal, _args: Vec<&str>) -> i32 {
    let fsw = crate::vfs::safe_wrap_fdfs(".".to_string()).0;
    // 1 = /.
    let vdent = fsw.vfd_as_dentry(&fsw.get_fd(1, 0).unwrap()).unwrap();
    for n in vdent.get_entries() {
        term.writeln(&format!("VDE inode={},filename={}", n.inum, n.filename));
    }
    return 0;
}
