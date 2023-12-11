use wasm_bindgen::prelude::*;
use xterm_js_rs::Terminal;
use colored::Colorize;

#[wasm_bindgen(raw_module = "../../src/stores")]
extern "C" {
    fn wasmGetHome() -> i32;
}

pub fn exit(term: &Terminal, _args: Vec<&str>) -> i32 {
    crate::kmessage(term, "The system is going down for system halt NOW!");
    wasmGetHome();
    return 0;
}

pub fn nano(term: &Terminal, _args: Vec<&str>) -> i32 {
    term.writeln(&format!("{}", "Vim, Emacs, and ed are free...".bright_red().bold()));
    return 127;
}