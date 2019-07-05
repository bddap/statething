//! lib that gets compiled to wasm

#![no_std]

mod transition;

#[cfg(target_arch = "wasm32")]
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn transition(a: u32) -> u32 {
    crate::transition::transition(a)
}

#[no_mangle]
pub extern "C" fn version() -> u32 {
    crate::transition::version()
}

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn on_panic(_p: &PanicInfo) -> ! {
    loop {}
}
