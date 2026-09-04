// idt, stable-compatible (no x86-interrupt ABI)

use core::arch::asm;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}

#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u64,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry {
    offset_low: 0,
    selector: 0,
    ist: 0,
    type_attr: 0,
    offset_mid: 0,
    offset_high: 0,
    zero: 0,
}; 256];

fn set_gate(n: usize, addr: u64) {
    unsafe {
        IDT[n].offset_low = (addr & 0xFFFF) as u16;
        IDT[n].selector = 0x28;
        IDT[n].ist = 0;
        IDT[n].type_attr = 0x8E;
        IDT[n].offset_mid = ((addr >> 16) & 0xFFFF) as u16;
        IDT[n].offset_high = ((addr >> 32) & 0xFFFFFFFF) as u32;
        IDT[n].zero = 0;
    }
}

// fatal handlers never return, so plain extern C + hlt is fine on stable
extern "C" fn div0_handler() {
    crate::println!("EXC: divide error");
    loop {
        unsafe { asm!("hlt") };
    }
}

extern "C" fn gp_handler() {
    crate::println!("EXC: general protection fault");
    loop {
        unsafe { asm!("hlt") };
    }
}

extern "C" fn pf_handler() {
    crate::println!("EXC: page fault");
    loop {
        unsafe { asm!("hlt") };
    }
}

extern "C" fn df_handler() {
    crate::println!("EXC: double fault");
    loop {
        unsafe { asm!("hlt") };
    }
}

// breakpoint returns via iretq, naked so we control it
#[unsafe(naked)]
extern "C" fn bp_stub() {
    core::arch::naked_asm!("iretq");
}

pub unsafe fn init() {
    set_gate(0, div0_handler as *const () as u64);
    set_gate(3, bp_stub as *const () as u64);
    set_gate(8, df_handler as *const () as u64);
    set_gate(13, gp_handler as *const () as u64);
    set_gate(14, pf_handler as *const () as u64);
    let ptr = IdtPtr {
        limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
        base: &raw const IDT as *const _ as u64,
    };
    asm!("lidt [{0}]", in(reg) &ptr);
}
