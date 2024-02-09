use xterm_js_rs::Terminal;

const LN_HELP: &str = "Usage: ln [OPTION]... <TARGET> <LINK_NAME>
Create a link to TARGET with the name LINK_NAME.
Create hard links by default, symbolic links with --symbolic.
By default, each destination (name of new link) should not already exist.
When creating hard links, each TARGET must exist. Symbolic links
can hold arbitrary text; if later resolved, a relative link is
interpreted in relation to its parent directory.

  -f, --force    remove destination file if it exists
  -s, --symbolic make symbolic links instead of hard links
";
const LN_VSN: &str = "ln (IrisOS-nano) 0.1
Copyright (C) Amy Parker, 2024
License AGPLv3+: GNU AGPL version 3 or later <https://gnu.org/licenses/agpl.html>
This is free software; you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Amy Parker <amy@amyip.net>.
Based on ln by Mike Parker and David MacKenzie.";
struct LnOpts {
    force: bool,
    symbolic: bool,
    help: bool,
    version: bool,
}

pub fn ln(term: &Terminal, args: Vec<&str>) -> i32 {
    if args.len() < 1 {
        term.writeln("ln: missing file operand");
        crate::common::minfo(term, "ln");
        return -1;
    }
    let mut split = args.len();
    for n in 0..args.len() {
        if !args[n].starts_with('-') {
            split = n;
            break;
        }
    }
    let (opts, files) = args.split_at(split);
    let mut opt = LnOpts {
        force: false,
        symbolic: false,
        help: false,
        version: false,
    };
    for arg in opts {
        if arg.len() <= 1 {
            term.writeln("ln: unknown argument");
            crate::common::minfo(term, "ln");
            return -2;
        }
        if arg.chars().take(2).last().unwrap() == '-' {
            match *arg {
                "--help" => opt.help = true,
                "--version" => opt.help = true,
                "--force" => opt.force = true,
                "--symbolic" => opt.symbolic = true,
                _ => {
                    term.writeln(&format!("ln: unrecognized option '{}'", arg));
                    crate::common::minfo(term, "ln");
                    return -5;
                }
            }
            continue;
        }
        for n in arg.get(1..).unwrap().chars() {
            match n {
                'f' => opt.force = true,
                's' => opt.symbolic = true,
                _ => {
                    term.writeln(&format!("ln: invalid option -- '{}'", n));
                    crate::common::minfo(term, "ln");
                    return -3;
                }
            }
        }
    }
    if opt.help {
        term.writeln(LN_HELP);
        return 0;
    }
    if opt.version {
        term.writeln(LN_VSN);
        return 0;
    }
    if files.len() > 2 {
        term.writeln("ln: too many arguments");
        crate::common::minfo(term, "ln");
        return -6;
    }
    if files.len() < 2 {
        term.writeln("ln: not enough arguments");
        crate::common::minfo(term, "ln");
        return -7;
    }
    let check = crate::vfs::futils::find_file(files[1].to_string(), false);
    let mut rsn = files[1].rsplitn(2, '/');
    let fx = rsn.next().unwrap().to_string();
    let pino = crate::vfs::futils::find_file(rsn.next().unwrap_or(".").to_string(), false)
        .left()
        .unwrap();
    if check.is_left() {
        if !opt.force {
            term.writeln(&format!(
                "ln: failed to create link '{}': File exists",
                files[1]
            ));
            return -4;
        }
        let f = check.left().unwrap();
        f.0.delete_file(f.1.get_inum(), pino.1.get_inum()).unwrap();
    }
    if opt.symbolic {
        let ino = pino
            .0
            .create_file(pino.1.get_inum(), fx, files[0].as_bytes())
            .unwrap();
        let fd = pino.0.get_fd(ino, 0).unwrap();
        pino.0.chmod(&fd, 0x2000 + 0o777);
    } else {
        let tgt = u32::from_le_bytes(
            crate::vfs::futils::find_file(files[0].to_string(), true)
                .right()
                .unwrap()
                .unwrap()
                .try_into()
                .unwrap(),
        );
        pino.0.hardlink(pino.1.get_inum(), tgt, fx);
    }
    return 0;
}
