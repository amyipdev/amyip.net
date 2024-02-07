use xterm_js_rs::Terminal;

pub fn pwd(term: &Terminal, _args: Vec<&str>) -> i32 {
    if _args.len() != 0 {
        term.writeln("pwd: warn: implementation does not support arguments");
    }
    let mut r = crate::sysvars::load_cwd();
    if r.ends_with('/') && r.len() > 1 {
        r.pop();
    }
    term.writeln(&r);
    return 0;
}
