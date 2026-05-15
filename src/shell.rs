// tiny shell

use crate::{console, println, print};

fn parse_line(line: &[u8]) -> (&str, &str) {
    // split at first space
    let mut split = 0;
    while split < line.len() && line[split] != b' ' {
        split += 1;
    }
    let cmd = core::str::from_utf8(&line[..split]).unwrap_or("");
    let mut rest_start = split;
    while rest_start < line.len() && line[rest_start] == b' ' {
        rest_start += 1;
    }
    let rest = core::str::from_utf8(&line[rest_start..]).unwrap_or("");
    (cmd, rest)
}

pub fn run() -> ! {
    println!("type 'help' for commands");
    let mut buf = [0u8; 128];
    loop {
        print!("> ");
        let len = unsafe { console::read_line(&mut buf) };
        if len == 0 {
            continue;
        }
        let line = &buf[..len];
        let (cmd, rest) = parse_line(line);
        match cmd {
            "help" => {
                println!("commands: help, echo, about, version, mem, clear, halt, reboot");
            }
            "echo" => {
                println!("{}", rest);
            }
            "about" => {
                println!("BlackBoldOS - minimal freestanding OS, Limine + serial shell");
            }
            "version" => {
                println!("BlackBoldOS v0.1-dev");
            }
            "mem" => {
                crate::mem::dump();
            }
            "clear" => {
                // ANSI clear, works on qemu stdio + most serial terms
                crate::print!("\x1b[2J\x1b[H");
            }
            "halt" => {
                println!("halting...");
                unsafe {
                    core::arch::asm!("cli");
                    loop {
                        core::arch::asm!("hlt");
                    }
                }
            }
            "reboot" => {
                println!("rebooting via 8042...");
                unsafe {
                    use core::arch::asm;
                    // try ps/2 controller reset, works on qemu. real hw untested.
                    asm!(
                        "mov dx, 0x64",
                        "mov al, 0xFE",
                        "out dx, al",
                        out("dx") _,
                        out("al") _,
                    );
                    // fallback: triple fault via bad idt?
                    // if still here, halt
                    asm!("cli");
                    loop {
                        asm!("hlt");
                    }
                }
            }
            "" => {}
            _ => {
                println!("unknown command: {}", cmd);
            }
        }
    }
}
