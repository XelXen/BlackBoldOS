#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod limine;
mod serial;

// placeholder entry, doesn't boot yet
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        serial::init();
        serial::write_str("B");
    }
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
