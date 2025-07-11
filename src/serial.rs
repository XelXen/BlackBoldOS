// serial COM1 first attempt

use core::arch::asm;

const COM1: u16 = 0x3F8;

#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val);
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("in al, dx", out("al") ret, in("dx") port);
    ret
}

pub unsafe fn init() {
    // disable interrupts
    outb(COM1 + 1, 0x00);
    // enable DLAB to set baud - using divisor 12 (9600?) should be 1 for 115200
    outb(COM1 + 3, 0x80);
    outb(COM1 + 0, 0x0C);
    outb(COM1 + 1, 0x00);
    // 8n1 without FIFO init? forgot fifo
    outb(COM1 + 3, 0x03);
    outb(COM1 + 4, 0x0B);
}

unsafe fn is_transmit_empty() -> bool {
    inb(COM1 + 5) & 0x20 != 0
}

pub unsafe fn write_byte(b: u8) {
    while !is_transmit_empty() {}
    outb(COM1, b);
}

pub unsafe fn write_str(s: &str) {
    for b in s.bytes() {
        write_byte(b);
    }
}
