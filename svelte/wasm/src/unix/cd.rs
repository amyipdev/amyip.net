use xterm_js_rs::Terminal;

pub fn cd(term: &Terminal, args: Vec<&str>) -> i32 {
    if args.len() == 0 {
        crate::sysvars::store_cwd("/".to_string());
        return 0;
    }
    if args.len() != 1 {
        term.writeln("cd: too many arguments");
        return 1;
    }
    let mut cwd = crate::sysvars::load_cwd();
    cwd.push_str(args[0]);
    if !cwd.ends_with("/") {
        cwd.push('/');
    }
    let r = crate::vfs::futils::find_file(cwd.clone(), false);
    if r.is_right() {
        term.writeln("cd: No such file or directory");
        return 1;
    }
    let r2 = r.unwrap_left();
    if r2.0.file_perms(&r2.1).unwrap() & 0xf000 != 0x1000 {
        term.writeln("cd: Not a directory");
        return 1;
    }
    crate::sysvars::store_cwd(cwd);
    return 0;
}
