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
            // backspace, don't delete prompt
            if len > 0 {
                len -= 1;
                // erase char on terminal
                serial::write_byte(0x08);
                serial::write_byte(b' ');
                serial::write_byte(0x08);
            }
        } else if b == 0x1B {
            // ignore escape (arrows send esc [ ...)
            continue;
        } else if b == 0 {
            continue;
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
