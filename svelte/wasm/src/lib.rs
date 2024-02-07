mod builtins;
mod common;
mod instant;
mod keys;
mod nanotools;
mod sysvars;
mod unix;
mod vfs;

use std::sync::atomic::{AtomicBool, Ordering};

use colored::Colorize;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;
use xterm_js_rs::addons::fit::FitAddon;
use xterm_js_rs::keys::BellStyle;
use xterm_js_rs::{OnKeyEvent, Terminal, TerminalOptions, Theme};

use keys::*;

static mut ADDON: Lazy<FitAddon> = Lazy::new(|| FitAddon::new());
static TSC: Lazy<instant::Instant> = Lazy::new(|| instant::Instant::now());
static IN_IRUN: AtomicBool = AtomicBool::new(true);

const MAX_HIST_LEN: usize = 1000;

#[wasm_bindgen]
pub fn fit() -> () {
    // because we aren't using any threading,
    // this is safe. if threading is implemented
    // in the future, use an external mutex-based lock
    // or a spinlock. Safety analysis: PASS
    unsafe { ADDON.fit() };
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// TODO: get rid of panics/?/unwraps whenever unnecessary/avoidable
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    colored::control::set_override(true);
    // TODO: custom panic handler with kmessage/kernel panic
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
            .with_convert_eol(true)
            .with_bell_style(BellStyle::Both)
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
    kmessage_instr(&term, "uname -a");
    kmessage(&term, "tsc: initialized TSC via performance_now");
    vfs::mount_dummy();
    kmessage(&term, "dummyfs: mounted initfs at /");
    term.writeln(&format!("Welcome to {}!", "IrisOS-nano".bright_green()));
    term.writeln(&format!("Type {} for a list of commands.", "help".bold()));
    term.writeln(&format!("Type {} to load the filesystem.", "setup".bold()));
    term.writeln(&format!(
        "Type {} for more information.",
        "iris-info".bold()
    ));
    let mut ps1: &str = "$ ";
    term.write(ps1);
    let mut cb: String = String::new();
    let mut cp: usize = 0;
    let st: Terminal = Terminal::from(term.clone());
    let mut hist: std::collections::VecDeque<String> = std::collections::VecDeque::new();
    let mut chp: usize = usize::MAX;
    let mut chp_ac: bool = false;

    // this callback is the primary code of irun
    let cb = Closure::wrap(Box::new(move |e: OnKeyEvent| {
        let ev = e.dom_event();
        if !IN_IRUN.load(Ordering::Relaxed) {
            kmessage(&term, "Kernel panic - piping stdin is not supported");
            panic!();
        }
        // TODO: implement https://gist.github.com/tuxfight3r/60051ac67c5f0445efee
        // TODO/BUG: don't print non-printable characters (breaks buffer horrendously)
        match ev.key_code() {
            KEY_ENTER => {
                if cb.len() != 0 {
                    for _ in cp..cb.len() {
                        term.write(CURSOR_RIGHT);
                    }
                    term.writeln("");
                    // TODO: store shell_instruction result for $?
                    run_shell_instruction(&term, &cb.trim());
                    if hist.len() == 0 || *hist.back().unwrap() != cb {
                        hist.push_back(cb.clone());
                    }
                    cb.clear();
                    term.write(ps1);
                    cp = 0;
                    chp = usize::MAX;
                    chp_ac = false;
                } else {
                    term.writeln("");
                    term.write(ps1);
                }
            }
            KEY_BACKSPACE => {
                if cp != 0 {
                    term.write(CURSOR_BACKSPACE);
                    cb.remove(cp - 1);
                    cp -= 1;
                    term.write(cb.get(cp..).unwrap());
                    term.write(CURSOR_RIGHT);
                    term.write(CURSOR_BACKSPACE);
                    for _ in cp..cb.len() {
                        term.write(CURSOR_LEFT);
                    }
                } else {
                    term.write(CURSOR_BELL);
                }
            }
            KEY_LEFT_ARROW => {
                if cp != 0 {
                    term.write(CURSOR_LEFT);
                    cp -= 1;
                } else {
                    term.write(CURSOR_BELL);
                }
            }
            KEY_UP_ARROW => {
                if chp == 0 {
                    term.write(CURSOR_BELL);
                } else if !chp_ac {
                    if hist.len() == 0 {
                        term.write(CURSOR_BELL);
                    } else {
                        rpos(&term, cp, &cb);
                        chp = hist.len() - 1;
                        chp_ac = true;
                        term.write(&hist[chp]);
                        hist.push_back(cb.clone());
                        cb = hist[chp].clone();
                        cp = cb.len();
                        if hist.len() > MAX_HIST_LEN {
                            chp -= 1;
                            hist.pop_front();
                        }
                    }
                } else {
                    rpos(&term, cp, &cb);
                    chp -= 1;
                    term.write(&hist[chp]);
                    cb = hist[chp].clone();
                    cp = cb.len();
                }
            }
            KEY_RIGHT_ARROW => {
                if cp < cb.len() {
                    term.write(CURSOR_RIGHT);
                    cp += 1;
                } else {
                    term.write(CURSOR_BELL);
                }
            }
            KEY_DOWN_ARROW => {
                if !chp_ac || chp == hist.len() - 1 {
                    term.write(CURSOR_BELL);
                } else {
                    rpos(&term, cp, &cb);
                    chp += 1;
                    term.write(&hist[chp]);
                    cb = hist[chp].clone();
                    cp = cb.len();
                }
            }
            KEY_C if ev.ctrl_key() => {
                term.writeln("^C");
                term.write(ps1);
                cb.clear();
                cp = 0;
                chp = usize::MAX;
                chp_ac = false;
            }
            KEY_L if ev.ctrl_key() => term.clear(),
            _ => {
                if !ev.alt_key() && !ev.ctrl_key() && !ev.meta_key() {
                    term.write(&ev.key());
                    cb.insert(cp, e.key().chars().next().unwrap());
                    cp += 1;
                    term.write(cb.get(cp..).unwrap());
                    for _ in cp..cb.len() {
                        term.write(CURSOR_LEFT);
                    }
                }
            }
        }
    }) as Box<dyn FnMut(_)>);
    st.on_key(cb.as_ref().unchecked_ref());
    cb.forget();
    // TODO: rootfs
    // TODO: help command
    // TODO: man pages
    // END IrisOS-nano

    // This only runs when the module is being initialized.
    // It will never be run on more than one thread - and
    // in fact will only be run once. Safety analysis: PASS
    unsafe {
        st.load_addon(ADDON.clone().dyn_into::<FitAddon>()?.into());
    }
    st.focus();
    Ok(())
}

type PathFn = fn(&Terminal, Vec<&str>) -> i32;
fn check_path(exec: &str) -> Option<PathFn> {
    match exec {
        "uname" => Some(unix::uname::uname),
        "cat" => Some(unix::cat::cat),
        "ls" => Some(unix::ls::ls),
        "kmsg" => Some(nanotools::kmsg),
        "exit" => Some(builtins::exit),
        "iris-info" => Some(nanotools::iris_info),
        "nano" => Some(builtins::nano),
        "sanity-checks.infs" => Some(nanotools::test_infs),
        "sanity-checks.readroot" => Some(nanotools::test_read_root),
        "loadwebroot" => Some(nanotools::loadwebroot),
        "setup" => Some(nanotools::setup),
        "cd" => Some(unix::cd::cd),
        "pwd" => Some(unix::pwd::pwd),
        "mv" => Some(unix::mv::mv),
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

fn kmessage_instr(term: &Terminal, instr: &str) -> () {
    term.write(&fmt_ktime());
    run_shell_instruction(term, instr);
}

// TODO: write kernel messages to ring buffer
fn kmessage(term: &Terminal, msg: &str) -> () {
    term.write(&fmt_ktime());
    term.writeln(msg);
}

fn fmt_ktime() -> String {
    let elapsed = TSC.elapsed();
    format!("[{:>5}.{:06}] ", elapsed.as_secs(), elapsed.subsec_micros())
}

fn rpos(term: &Terminal, cp: usize, cb: &String) {
    for _ in cp..cb.len() {
        term.write(CURSOR_RIGHT);
    }
    for _ in 0..cb.len() {
        term.write(CURSOR_BACKSPACE);
    }
}
