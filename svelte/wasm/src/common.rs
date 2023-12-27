use xterm_js_rs::Terminal;

pub fn minfo(term: &Terminal, pname: &str) {
    term.writeln(&format!("Try '{} --help' for more information.", pname));
}

pub const fn fastceildiv(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

pub fn bytes_to_string(bytes: &[u8]) -> String {
    for n in 0..bytes.len() {
        if bytes[n] == 0 {
            return String::from_utf8(bytes[0..n].to_vec()).unwrap();
        }
    }
    String::from_utf8(bytes.to_vec()).unwrap()
}
