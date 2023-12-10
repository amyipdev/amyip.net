use xterm_js_rs::Terminal;

pub fn minfo(term: &Terminal, pname: &str) {
    term.writeln(format!("Try '{} --help' for more information.", pname).as_str());
}
