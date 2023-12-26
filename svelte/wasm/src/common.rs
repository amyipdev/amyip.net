use xterm_js_rs::Terminal;

pub fn minfo(term: &Terminal, pname: &str) {
    term.writeln(&format!("Try '{} --help' for more information.", pname));
}

pub const fn fastceildiv(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}
