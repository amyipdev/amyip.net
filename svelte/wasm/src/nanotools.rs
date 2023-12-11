// The functions in this module are specific
// to IrisOS-nano. They should never be implemented
// on full releases of IrisOS, or any non-experimental OS
// for that matter.

use xterm_js_rs::Terminal;
use colored::Colorize;
use once_cell::sync::Lazy;

pub fn kmsg(term: &Terminal, args: Vec<&str>) -> i32 {
    crate::kmessage(term, &args.join(" "));
    return 0;
}

// This one could have a similar command on full Iris,
// but definitely not this particular implementation.
static INFO_MSG: Lazy<String> = Lazy::new(|| { format!(
"{} v{} @ {}
Built in {} by {} {}
Licensed under AGPLv3 {}

IrisOS-nano is a simple {} web operating system,
hosted right here on {}. There are two main goals for it:
1. Be able to have all the same functionality as the rest of
   amyip.net, but in a command-line format, and
2. Serve as a test platform for future apps targeting IrisOS,
   a {} Unix-like OS.
As such, this also serves as a copy of my resume, and more.

Source code: {}",
"IrisOS-nano".bold().bright_green(),
env!("CARGO_PKG_VERSION").bold(),
"amyip.net".bright_cyan(),
"Rust".bright_yellow(),
"Amy Parker".bold(),
"<amy@amyip.net>".bright_black(),
"<https://gnu.org/licenses/agpl.html>".bright_black(),
"Unix-like".bold(),
"amyip.net".bright_cyan(),
"future-development".bright_red(),
"<https://github.com/amyipdev/amyip.net>".bright_black()
)});
pub fn iris_info(term: &Terminal, _args: Vec<&str>) -> i32 {
    term.writeln(&INFO_MSG);
    return 0;
}