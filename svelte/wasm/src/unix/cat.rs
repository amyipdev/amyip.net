use xterm_js_rs::Terminal;

pub fn cat(term: &Terminal, mut args: Vec<&str>) -> i32 {
    if args.len() < 1 {
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
