use std::sync::atomic::AtomicU16;

pub(crate) static UMASK: AtomicU16 = AtomicU16::new(0o022);