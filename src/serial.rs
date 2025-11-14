// serial COM1

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
    outb(COM1 + 1, 0x00); // disable interrupts
    outb(COM1 + 3, 0x80); // enable DLAB
    outb(COM1 + 0, 0x01); // divisor 1 = 115200
    outb(COM1 + 1, 0x00);
    outb(COM1 + 3, 0x03); // 8n1
    outb(COM1 + 2, 0xC7); // fifo
    outb(COM1 + 4, 0x0B); // rts/dsr
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
        // qemu stdio needs \r\n, fix stair-step
        if b == b'\n' {
            write_byte(b'\r');
        }
        write_byte(b);
    }
}

use core::fmt::{self, Write};

struct Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { write_str(s) };
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    let mut w = Writer;
    let _ = w.write_fmt(args);
}

unsafe fn is_data_ready() -> bool {
    inb(COM1 + 5) & 0x01 != 0
}

pub unsafe fn read_byte() -> u8 {
    while !is_data_ready() {}
    inb(COM1)
}
