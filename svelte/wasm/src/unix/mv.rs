use xterm_js_rs::Terminal;

// Not UNIX compliant, no options support
// TODO: don't allow moves to somewhere that already exists

pub fn mv(term: &Terminal, args: Vec<&str>) -> i32 {
    if args.len() == 1 && args[0] == "--help" {
        term.writeln("Usage: mv [src] [dest]");
        return 0;
    }
    if args.len() < 2 {
        term.writeln("mv: missing file operand");
        crate::common::minfo(term, "mv");
        return -1;
    }
    if args.len() > 2 {
        term.writeln("mv: too many arguments");
        crate::common::minfo(term, "mv");
        return -1;
    }
    let pi: u32 = u32::from_le_bytes(
        crate::vfs::futils::find_file(
            args[0].rsplitn(2, '/').nth(1).unwrap_or(".").to_string(),
            true,
        )
        .right()
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap(),
    );
    // we don't short - we need the resulting FS
    let df = crate::vfs::futils::find_file(args[0].to_string(), false)
        .left()
        .unwrap();
    let di = df.1.get_inum();
    // TODO: optimize out this double rsplitn call
    let si: u32 = u32::from_le_bytes(
        crate::vfs::futils::find_file(
            args[1].rsplitn(2, '/').nth(1).unwrap_or(".").to_string(),
            true,
        )
        .right()
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap(),
    );
    let name = args[1].rsplitn(2, '/').next().unwrap().to_string();
    // this will fail if they aren't on the same fs - if the same inode num exists, this goes very bad
    // TODO: check that they are both actually on the same fs
    df.0.hardlink(si, di, name);
    df.0.delete_file(di, pi);
    return 0;
}
