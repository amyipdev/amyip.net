// The functions in this module are specific
// to IrisOS-nano. They should never be implemented
// on full releases of IrisOS, or any non-experimental OS
// for that matter.

use crate::errors::ar;
use crate::vfs::VirtualFileSystem;
use colored::Colorize;
use once_cell::sync::Lazy;
use wasm_bindgen::JsCast;
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

/*
pub fn loadwebroot(term: &Terminal, args: Vec<&str>) -> i32 {
    if args.len() < 1 {
        term.writeln("loadwebroot: no URL provided");
        return 1;
    }
    let r = web_sys::XmlHttpRequest::new().unwrap();
    r.open_with_async("GET", args[0], false);
    r.override_mime_type("text/plain; charset=x-user-defined");
    //r.set_response_type(web_sys::XmlHttpRequestResponseType::Arraybuffer);
    r.send().unwrap();
    if r.ready_state() == 4 {
        let resp = r
            .response()
            .unwrap()
            .dyn_into::<js_sys::JsString>()
            .unwrap();
        let mut v: Vec<u8> = vec![];
        for n in resp.iter() {
            v.push(n.to_le_bytes()[0]);
        }
        crate::vfs::mount_root(Box::new(
            crate::vfs::infs::FileSystem::from_bytes(&v).unwrap_or_else(|| {
                term.writeln("loadwebroot: something went wrong, failing safe");
                crate::vfs::infs::mknrfs(128, 4096, 4096)
            }),
        ));
    } else {
        panic!("ready state is wrong");
    }
    return 0;
}*/

pub fn loadwebroot(term: &Terminal, args: Vec<&str>) -> i32 {
    if args.len() < 1 {
        term.writeln("loadwebroot: no URL provided");
        return 1;
    }
    crate::vfs::mount_root(Box::new(
        crate::vfs::infs::FileSystem::from_bytes(&ar!(
            binfetch_wasm::basic_fetch(args[0]),
            ah,
            -8,
            term
        ))
        .unwrap_or_else(|| {
            term.writeln("loadwebroot: something went wrong, failing safe");
            crate::vfs::infs::mknrfs(128, 4096, 1024)
        }),
    ));
    return 0;
}

pub fn setup(term: &Terminal, args: Vec<&str>) -> i32 {
    loadwebroot(term, vec!["/build/i.iar"]);
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

const HELPMSG: &str = "IrisOS-nano irun, version 0.1 (wasm32)
These commands are built in to irun. Other programs traverse the PATH.
Type `NAME --help` to find out more about the command `NAME`.

cat          [OPTS].. <FILE>..  ls     [DIRNAME]
cd           [DIR]              mkdir  <DIRNAME>
cp           <SRC> <DEST>       mv     <SRC> <DEST>
echo         [MSG]              pwd
exit                            rm     <FILE>
help                            rmdir  <DIRECTORY>
iris-info                       setup
kmsg         [MSG]              touch  <FILENAME>
ln           <TARGET> <NAME>    uname  [OPTIONS]
loadwebroot  [URL]";
pub fn help(term: &Terminal, _args: Vec<&str>) -> i32 {
    term.writeln(HELPMSG);
    return 0;
}

pub fn neofetch(term: &Terminal, _args: Vec<&str>) -> i32 {
    term.writeln(&format!("
[0;34;40m                                        [35;49;1m   root@amyip.net
[0;34;40m                                        [37;49m   --------------
[0;34;40m       [0;1;37;47m                         [0;34;40m        [35;49;1m   OS[37;49m: IrisOS-nano
[0;34;40m     [0;5;37;47m                              [0;31;40m     [35;49;1m   Kernel[37;49m: {kvsn}
[0;34;40m  [0;5;33;47m                                    [0;34;40m  [35;49;1m   Uptime[37;49m: {cup}
[0;34;40m [0;1;30;47m     [0;5;35;40m      [0;1;30;47m                [0;32;40m      [0;1;37;47m     [0;34;40m [35;49;1m   Shell[37;49m: irun 0.1
[0;5;36;40m [0;5;37;47m    [0;1;30;40m      [0;5;37;47m                  [0;5;37;40m      [0;1;37;47m    [0;5;33;40m [35;49;1m   CPU[37;49m: wasm32
[0;1;37;47m    [0;5;35;40m      [0;5;37;47m    [0;5;37;40m           [0;5;35;40m [0;5;37;47m    [0;1;30;40m      [0;5;37;47m    [35;49;1m   Memory[37;49m: {musage}MiB / 4096MiB
[0;5;37;47m    [0;32;40m      [0;5;37;47m   [0;5;37;40m     [0;1;30;47m    [0;32;40m     [0;5;37;47m   [0;5;35;40m      [0;1;30;47m    [35;49;1m   Build[37;49m: {gitv}
[0;5;37;47m    [0;32;40m      [0;5;37;47m   [0;5;37;40m     [0;1;30;47m    [0;32;40m     [0;5;37;47m   [0;5;35;40m      [0;1;30;47m    [35;49;1m   Rust[37;49m: v{rustvsn}
[0;1;37;47m    [0;5;35;40m      [0;5;37;47m    [0;5;37;40m           [0;5;35;40m [0;5;37;47m    [0;1;30;40m      [0;5;37;47m    [35;49;1m
[0;5;36;40m [0;5;37;47m    [0;1;30;40m      [0;5;37;47m                  [0;5;37;40m      [0;1;37;47m    [0;5;33;40m [35;49;1m   Mastodon[37;49m: \x1b]8;id=mastodonlink;https://blahaj.zone/@amyipdev\x07@amyipdev@blahaj.zone\x1b]8;;\x07
[0;34;40m [0;1;30;47m     [0;5;35;40m      [0;1;30;47m                [0;32;40m      [0;1;37;47m     [0;34;40m [35;49;1m   Matrix[37;49m: \x1b]8;id=matrixlink;https://matrix.to/#/@amyipdev1:matrix.org\x07@amyipdev1:matrix.org\x1b]8;;\x07
[0;34;40m  [0;5;33;47m                                    [0;34;40m  [35;49;1m   Instagram[37;49m: \x1b]8;id=instagramlink;https://instagram.com/amyipdev\x07@amyipdev\x1b]8;;\x07
[0;34;40m     [0;5;37;47m                              [0;31;40m     [35;49;1m   Discord[37;49m: \x1b]8;id=discordsitelink;https://discord.com\x07@amyipdev\x1b]8;;\x07
[0;34;40m       [0;1;37;47m                         [0;34;40m        [35;49;1m
[0;34;40m                                        [0m   \x1b[40m    \x1b[41m    \x1b[42m    \x1b[43m    \x1b[44m    \x1b[45m    \x1b[46m    \x1b[47m    \x1b[0m
[0;34;40m                                        [0m   \x1b[48;5;8m    \x1b[48;5;9m    \x1b[48;5;10m    \x1b[48;5;11m    \x1b[48;5;12m    \x1b[48;5;13m    \x1b[48;5;14m    \x1b[47;15m    \x1b[0m
", kvsn = env!("CARGO_PKG_VERSION"), cup = timeconv(crate::instant::Instant::now().i()), musage = wasm_bindgen::memory().unchecked_into::<js_sys::WebAssembly::Memory>().grow(0) >> 4, gitv = git_version::git_version!(), rustvsn = env!("RUSTC_VERSION")));
    return 0;
}

fn timeconv(mut n: u64) -> String {
    n /= 60000000;
    let mins = n % 60;
    n /= 60;
    let hrs = n % 60;
    n /= 60;
    let days = n / 24;
    let mut b = String::new();
    if days != 0 {
        b.push_str(&days.to_string());
        b.push_str(" days, ");
    }
    if hrs != 0 {
        b.push_str(&hrs.to_string());
        b.push_str(" hours, ");
    }
    b.push_str(&mins.to_string());
    b.push_str(" mins");
    b
}

fn ah(term: &Terminal, code: i32) {
    term.writeln(match code {
        -8 => "loadwebroot: could not load root: network error occurred",
        _ => "nanotools: unknown error",
    });
}
