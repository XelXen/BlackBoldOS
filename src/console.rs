// console line input with bounds

use crate::serial;

pub unsafe fn read_line(buf: &mut [u8]) -> usize {
    let mut len = 0;
    loop {
        let b = serial::read_byte();
        if b == b'\r' || b == b'\n' {
            serial::write_str("\n");
            break;
        } else if b == 0x08 || b == 0x7F {
            if len > 0 {
                len -= 1;
                serial::write_str("\x08 \x08");
            }
        } else {
            if len < buf.len() {
                buf[len] = b;
                len += 1;
                serial::write_byte(b);
            }
            // else truncate, ignore extra
        }
    }
    len
}
