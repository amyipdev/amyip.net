// The functions in this module are specific
// to IrisOS-nano. They should never be implemented
// on full releases of IrisOS, or any non-experimental OS
// for that matter.

use xterm_js_rs::Terminal;

pub fn kmsg(term: &Terminal, args: Vec<&str>) -> i32 {
    crate::kmessage(term, &args.join(" "));
    return 0;
}
