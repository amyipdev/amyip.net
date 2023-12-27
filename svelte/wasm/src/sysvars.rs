use std::sync::atomic::AtomicU16;

use once_cell::sync::Lazy;

pub(crate) static UMASK: AtomicU16 = AtomicU16::new(0o022);

// on other systems, CWD ends without /
// here, CWD ends with /
static mut CWD: Lazy<String> = Lazy::new(|| "/".to_string());
pub fn load_cwd() -> String {
    unsafe { CWD.clone() }
}
pub fn store_cwd(s: String) {
    unsafe { *CWD = s }
}
