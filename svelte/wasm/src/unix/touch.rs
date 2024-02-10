use xterm_js_rs::Terminal;

// avoiding options again bc who needs that
// TODO: instead of just creating previously nonexistent files,
// TODO: update the modified identifier on them,
// TODO: which requires more VFS extensions
pub fn touch(term: &Terminal, args: Vec<&str>) -> i32 {
    if args.len() > 1 {
        term.writeln("touch: too many arguments");
        crate::common::minfo(term, "touch");
        return -1;
    }
    if args.len() < 1 {
        term.writeln("touch: missing file operand");
        crate::common::minfo(term, "touch");
        return -1;
    }
    if args[0] == "--help" {
        term.writeln("Usage: touch [target]");
        return 0;
    }
    let tgt = crate::vfs::futils::find_file(args[0].to_string(), false);
    if tgt.is_left() {
        term.writeln("touch: updating mtime not yet supported");
        return 127;
    }
    let mut rsn = args[0].rsplitn(2, '/');
    let ntgt = rsn.next().unwrap().to_string();
    let dir = rsn.next().unwrap_or(".").to_string();
    let pino = crate::vfs::futils::find_file(dir, false).left().unwrap();
    if pino.0.create_file(pino.1.get_inum(), ntgt, &[0u8; 0]).is_none() {
        term.writeln("touch: could not touch file: read-only filesystem");
        return -3;
    }
    return 0;
}
