#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod limine;
mod serial;
mod gdt;
mod idt;
mod console;
mod shell;

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
        gdt::init();
        idt::init();
        core::arch::asm!("int3");
    }
    println!("BlackBoldOS v0.1");
    println!("boot ok");
    println!("idt ok");
    shell::run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // best effort print
    print!("PANIC: {}\n", info);
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
