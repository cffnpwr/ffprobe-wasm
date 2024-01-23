use std::panic;

pub(crate) fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
