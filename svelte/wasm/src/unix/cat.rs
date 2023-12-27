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
        term.writeln(
            &String::from_utf8(crate::vfs::futils::read_to_end(f.to_string()).unwrap()).unwrap(),
        );
    }
    return 0;
}
