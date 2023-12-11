use crate::common;

use xterm_js_rs::Terminal;

const UNAME_KERN: &str = "IrisOS-nano";
const UNAME_KVSN: &str = env!("CARGO_PKG_VERSION");
const UNAME_NODENAME: &str = "amyip.net";
// This is valid because WASM is single-threaded,
// so only one processor, no SMT. We also don't
// actually need to bake in compile info...
const UNAME_REL: &str = "#1";
const UNAME_OS: &str = "IrisOS";
// If wasm64 becomes a thing in the future, this needs
// to be conditional
const UNAME_MACH: &str = "wasm32";
const UNAME_HELP: &str = "Usage: uname [OPTION]...
Print certain system information. With no OPTION, same as -s.

 -a, --all              print all information, in the following order:
 -s, --kernel-name      print the kernel name
 -n, --nodename         print the hostname
 -r, --kernel-release   print the kernel release
 -v, --kernel-version   print the kernel version
 -m, --machine          print the machine hardware name
 -o, --operating-system print the operating system
     --help             display this help and exit
     --version          output version information and exit
";
const UNAME_VSN_TXT: &str = "uname (IrisOS-nano) 0.1
Copyright (C) Amy Parker, 2023
License AGPLv3+: GNU AGPL version 3 or later <https://gnu.org/licenses/agpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Amy Parker <amy@amyip.net>.
Based on uname by David MacKenzie.";
struct UnameArgs {
    /// All information
    pub a: bool,
    /// Kernel name
    pub s: bool,
    /// Hostname
    pub n: bool,
    /// Kernel release (actual version)
    pub r: bool,
    /// Kernel version (compile info)
    pub v: bool,
    /// Machine hardware name
    pub m: bool,
    /// Full OS name
    pub o: bool,
    /// Print help
    pub help: bool,
    /// Print version
    pub vsn: bool,
}
pub fn uname(term: &Terminal, args: Vec<&str>) -> i32 {
    let mut p: UnameArgs = UnameArgs {
        a: false,
        s: false,
        n: false,
        r: false,
        v: false,
        m: false,
        o: false,
        help: false,
        vsn: false,
    };
    for arg in &args {
        if !arg.starts_with('-') || arg.len() <= 1 {
            term.writeln(format!("uname: extra operand '{}'", arg).as_str());
            common::minfo(term, "uname");
            return 1;
        }
        if arg.chars().take(2).last().unwrap() == '-' {
            match *arg {
                "--help" => p.help = true,
                "--version" => p.vsn = true,
                "--all" => p.a = true,
                "--kernel-name" => p.s = true,
                "--nodename" => p.n = true,
                "--kernel-release" => p.r = true,
                "--kernel-version" => p.v = true,
                "--machine" => p.m = true,
                "--operating-system" => p.o = true,
                _ => {
                    term.writeln(format!("uname: unrecognized option '{}'", arg).as_str());
                    common::minfo(term, "uname");
                    return 1;
                }
            }
            continue;
        }
        for n in arg.get(1..).unwrap().chars() {
            match n {
                'a' => p.a = true,
                's' => p.s = true,
                'n' => p.n = true,
                'r' => p.r = true,
                'v' => p.v = true,
                'm' => p.m = true,
                'o' => p.o = true,
                _ => {
                    term.writeln(format!("uname: invalid option -- '{}'", { n }).as_str());
                    common::minfo(term, "uname");
                    return 2;
                }
            }
        }
    }
    if p.help {
        term.writeln(UNAME_HELP);
        return 0;
    }
    if p.vsn {
        term.writeln(UNAME_VSN_TXT);
        return 0;
    }
    let mut first: bool = true;
    if p.s || args.len() == 0 || p.a {
        first = false;
        term.write_callback(UNAME_KERN, &js_sys::Function::new_no_args(""));
    }
    if p.n || p.a {
        if first {
            first = false;
        } else {
            term.write(" ");
        }
        term.write(UNAME_NODENAME);
    }
    if p.r || p.a {
        if first {
            first = false;
        } else {
            term.write(" ");
        }
        term.write(UNAME_KVSN);
    }
    if p.v || p.a {
        if first {
            first = false;
        } else {
            term.write(" ");
        }
        term.write(UNAME_REL);
    }
    if p.m || p.a {
        if first {
            first = false;
        } else {
            term.write(" ");
        }
        term.write(UNAME_MACH);
    }
    if p.o || p.a {
        if !first {
            term.write(" ")
        }
        term.write(UNAME_OS);
    }
    term.writeln("");
    return 0;
}
