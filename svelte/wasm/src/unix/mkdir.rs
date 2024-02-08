use xterm_js_rs::Terminal;

pub fn mkdir(term: &Terminal, args: Vec<&str>) -> i32 {
    if args.len() == 0 {
        term.writeln("mkdir: missing directory operand");
        return -1;
    }
    if args.len() >= 2 {
        term.writeln("mkdir: too many arguments");
        return -1;
    }
    if args[0] == "--help" {
        term.writeln("Usage: mkdir [target]");
        return 0;
    }
    let mut parts: Vec<&str> = args[0].rsplitn(2, '/').collect();
    if parts.len() == 1 {
        parts.push(".");
    }
    let pino = crate::vfs::futils::find_file(parts[1].to_string(), false)
        .left()
        .unwrap();
    pino.0
        .create_directory(pino.1.get_inum(), parts[0].to_string());
    return 0;
}
