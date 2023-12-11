use wasm_bindgen::prelude::*;
use xterm_js_rs::Terminal;

#[wasm_bindgen(raw_module = "../../src/stores")]
extern "C" {
    fn wasmGetHome() -> i32;
}

pub fn exit(term: &Terminal, args: Vec<&str>) -> i32 {
    crate::kmessage(term, "The system is going down for system halt NOW!");
    wasmGetHome();
    return 0;
}
