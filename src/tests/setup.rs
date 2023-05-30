use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

#[allow(dead_code)]
pub struct Setup {
    ready: AtomicBool,
}

impl Setup {
    #[allow(dead_code)]
    pub fn init(&mut self) {
        if !(self.ready.load(Ordering::Relaxed)) {
            env::set_var("RUST_LOG", "debug");
            env::set_var("RUST_BACKTRACE", "1");
            env_logger::init();
            self.ready.store(true, Ordering::Relaxed);
        }
    }
}

#[allow(dead_code)]
static mut SETUP: Setup = Setup {
    ready: AtomicBool::new(false),
};

#[allow(dead_code)]
static INIT: Once = Once::new();
/// Setup function that is only run once, even if called multiple times.
#[allow(dead_code)]
pub fn setup() {
    INIT.call_once(|| {
        unsafe { SETUP.init() };
    });
}