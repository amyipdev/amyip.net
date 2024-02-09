use xterm_js_rs::Terminal;

pub fn rm(term: &Terminal, args: Vec<&str>) -> i32 {
    // TODO: dedup this logic with other commands like touch
    if args.len() > 1 {
        term.writeln("rm: too many arguments");
        crate::common::minfo(term, "rm");
        return -1;
    }
    if args.len() < 1 {
        term.writeln("rm: missing file operand");
        crate::common::minfo(term, "rm");
        return -1;
    }
    if args[0] == "--help" {
        term.writeln("Usage: rm [target]");
        return 0;
    }
    let mut f = match crate::vfs::futils::find_file(args[0].to_string(), false).left() {
        Some(v) => v,
        None => {
            term.writeln(&format!(
                "rm: cannot remove '{}': No such file or directory",
                args[0]
            ));
            return 1;
        }
    };
    if f.0.file_perms(&f.1).unwrap() & 0xf000 == 0x1000 {
        term.writeln(&format!("rm: cannot remove '{}': Is a directory", args[0]));
        return 2;
    }
    let pino = u32::from_le_bytes(
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
    f.0.delete_file(f.1.get_inum(), pino).unwrap();
    return 0;
}
