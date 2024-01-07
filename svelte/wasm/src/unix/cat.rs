use xterm_js_rs::Terminal;

const CAT_HELP: &str = "Usage: cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

Standard input mode is not supported.

 -A, --show-all         equivalent to -vET
 -b, --number-nonblank  number nonempty output lines, overrides -n
 -e                     equivalent to -vE
 -E, --show-ends        display $ at the end of each line
 -n, --number           number all output lines
 -s, --squeeze-blank    suppress repeated output lines
 -t                     equivalent to -vT
 -T, --show-tabs        display TAB characters as ^I
 -v, --show-nonprinting use ^ notation (except for TAB)
     --help             display this help and exit
     --version          output version information and exit

Examples:
  cat f g 	Output f's contents, then g's contents.";
const CAT_VSN: &str = "cat (IrisOS-nano) 0.1
Copyright (C) Amy Parker, 2023
License AGPLv3+: GNU AGPL version 3 or later <https://gnu.org/licenses/agpl.html>
This is free software; you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Amy Parker <amy@amyip.net>.
Based on cat by Torbjorn Granlund and Richard M. Stallman.";
struct CatOpts {
    number_nonblank: bool,
    show_ends: bool,
    number: bool,
    squeeze_blank: bool,
    show_tabs: bool,
    show_nonprinting: bool,
    help: bool,
    version: bool,
}
// TODO: is there a better way to dedup with what is in uname so far?
pub fn cat(term: &Terminal, mut args: Vec<&str>) -> i32 {
    if args.len() < 1 {
        term.writeln("cat: stdin mode not supported in cat-irun");
        crate::common::minfo(term, "cat");
        return -1;
    }
    let mut split = args.len();
    for n in 0..args.len() {
        if !args[n].starts_with('-') {
            split = n;
            break;
        }
    }
    let (opts, files) = args.split_at_mut(split);
    let mut opt = CatOpts {
        // if set, unset number
        number_nonblank: false,
        show_ends: false,
        number: false,
        squeeze_blank: false,
        show_tabs: false,
        // "use ^ and M- notation, except for LFD and TAB" (no clue what LFD is)
        // TAB should be ^I, no clue what that's about
        // map for caret notation: http://xahlee.info/comp/ascii_chars.html
        show_nonprinting: false,
        help: false,
        version: false,
    };
    for arg in opts {
        if arg.len() <= 1 {
            // must be "-"
            term.writeln("cat: stdin file ('-') not supported in cat-irun");
            crate::common::minfo(term, "cat");
            return -1;
        }
        if arg.chars().take(2).last().unwrap() == '-' {
            match *arg {
                "--help" => opt.help = true,
                "--version" => opt.version = true,
                "--show-all" => {
                    opt.show_nonprinting = true;
                    opt.show_ends = true;
                    opt.show_tabs = true;
                }
                "--number-nonblank" => opt.number_nonblank = true,
                "--show-ends" => opt.show_ends = true,
                "--number" => opt.number = true,
                "--squeeze-blank" => opt.squeeze_blank = true,
                "--show-tabs" => opt.show_tabs = true,
                "--show-nonprinting" => opt.show_nonprinting = true,
                _ => {
                    term.writeln(&format!("cat: unrecognized option '{}'", arg));
                    crate::common::minfo(term, "cat");
                    return 1;
                }
            }
            continue;
        }
        for n in arg.get(1..).unwrap().chars() {
            match n {
                'A' => {
                    opt.show_nonprinting = true;
                    opt.show_ends = true;
                    opt.show_tabs = true;
                }
                'b' => opt.number_nonblank = true,
                'e' => {
                    opt.show_nonprinting = true;
                    opt.show_ends = true;
                }
                'E' => opt.show_ends = true,
                'n' => opt.number = true,
                's' => opt.squeeze_blank = true,
                't' => {
                    opt.show_nonprinting = true;
                    opt.show_tabs = true;
                }
                'T' => opt.show_tabs = true,
                // does nothing
                'u' => (),
                'v' => opt.show_nonprinting = true,
                _ => {
                    term.writeln(&format!("cat: invalid option -- '{}'", n));
                    crate::common::minfo(term, "cat");
                    return 2;
                }
            }
        }
    }
    if opt.help {
        term.writeln(CAT_HELP);
        return 0;
    }
    if opt.version {
        term.writeln(CAT_VSN);
        return 0;
    }
    if opt.number_nonblank {
        opt.number = false;
    }
    let mut cl = 1;
    let mut pe = true;
    for f in files {
        let txt = crate::vfs::futils::read_to_end(f.to_string(), false);
        if txt.is_none() {
            term.writeln(&format!("cat: {}: No such file or directory", f));
            return -2;
        }
        let out = String::from_utf8(txt.unwrap())
            .unwrap()
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        for line in out {
            if opt.squeeze_blank {
                if pe && line == "" {
                    continue;
                }
                pe = line == "";
            }
            if opt.number || (opt.number_nonblank && line != "") {
                term.write(&format!("{:>6}\t", cl));
                cl += 1;
            }
            if !(opt.show_nonprinting || opt.show_tabs) {
                term.write(&line);
            } else {
                let mut tmp: String;
                for c in line.chars() {
                    term.write({
                        if c == '\t' {
                            if opt.show_tabs {
                                "^I"
                            } else {
                                "\t"
                            }
                        } else if (c as u8) <= 31 {
                            if opt.show_nonprinting {
                                tmp = (((c as u8) + 64) as char).to_string();
                                &tmp
                            } else {
                                ""
                            }
                        } else if (c as u8) == 127 {
                            if opt.show_nonprinting {
                                "^?"
                            } else {
                                ""
                            }
                        } else {
                            tmp = c.to_string();
                            &tmp
                        }
                    });
                }
            }
            term.writeln(if opt.show_ends { "$" } else { "" });
        }
    }
    return 0;
}
