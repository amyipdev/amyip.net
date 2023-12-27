use xterm_js_rs::Terminal;


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
    let mut split = args.len() - 1;
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
    // TODO: actually respect cat options
    for f in files {
		let txt = crate::vfs::futils::read_to_end(f.to_string());
		if txt.is_none() {
			term.writeln(&format!("cat: {}: No such file or directory", f));
			return -2;
		}
        term.writeln(
            &String::from_utf8(txt.unwrap()).unwrap(),
        );
    }
    return 0;
}
