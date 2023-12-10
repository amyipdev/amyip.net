mod common;
mod unix;

use wasm_bindgen::prelude::*;

use xterm_js_rs::addons::fit::FitAddon;
use xterm_js_rs::{Terminal, TerminalOptions, Theme};

use once_cell::sync::Lazy;

static mut ADDON: Lazy<FitAddon> = Lazy::new(|| FitAddon::new());

#[wasm_bindgen]
pub fn fit() {
    // because we aren't using any threading,
    // this is safe. if threading is implemented
    // in the future, use an external mutex-based lock
    // or a spinlock. Safety analysis: PASS
    unsafe { ADDON.fit() };
}

// TODO: implement auto-resizer
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    //addon = FitAddon::new();
    let term: Terminal = Terminal::new(
        TerminalOptions::new()
            .with_cursor_blink(true)
            .with_cursor_width(10)
            .with_font_size(20)
            .with_right_click_selects_word(true)
            .with_draw_bold_text_in_bright_colors(true)
            .with_font_family("Inconsolata")
            .with_theme(
                Theme::new()
                    .with_foreground("#f5f1e3")
                    .with_background("#191a22"),
            ),
    );
    let el = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("terminal")
        .unwrap();
    term.open(el.dyn_into()?);

    // The following block is the official definition
    // for IrisOS-nano. Initially, this is just going to
    // be a cool terminal applet; however, it may eventually
    // be able to emulate the running of some IrisOS programs.
    // Note that some functions are defined outside for convenience.

    // BEGIN IrisOS-nano
    run_shell_instruction(&term, "uname -a");
    // TODO: initialize clock (tsc)
    // TODO: kmessage()? print timestamps like in dmesg?
    // TODO: /sbin/login-style welcome, info printing
    // TODO: exit command
    // TODO: rootfs
    // TODO: colored?
    // TODO: actually interactive shell from example
    // TODO: help command
    // END IrisOS-nano

    // This only runs when the module is being initialized.
    // It will never be run on more than one thread - and
    // in fact will only be run once. Safety analysis: PASS
    unsafe {
        term.load_addon(ADDON.clone().dyn_into::<FitAddon>()?.into());
    }
    term.focus();
    Ok(())
}

type PathFn = fn(&Terminal, Vec<&str>) -> i32;
fn check_path(exec: &str) -> Option<PathFn> {
    match exec {
        "uname" => Some(unix::uname::uname),
        _ => None,
    }
}

// We don't need to support complex shell instructions...
fn run_shell_instruction(term: &Terminal, instr: &str) -> i32 {
    let mut v: Vec<&str> = instr.split(" ").collect();
    match check_path(v[0]) {
        Some(f) => f(term, v.drain(1..).collect()),
        None => {
            // in the future, this will search the instance rootfs
            // for now, that's not yet implemented, so we just complain
            term.writeln(format!("irun: {}: command not found...", v[0]).as_str());
            return 127;
        }
    }
}
