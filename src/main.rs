#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod limine;
mod serial;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::serial::_print(core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", core::format_args!($($arg)*))
    };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        serial::init();
    }
    println!("BlackBoldOS v0.0 - hello serial");
    println!("boot ok");
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // best effort print
    print!("PANIC: {}\n", info);
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
