// gdt, first try - probably wrong access bytes?

use core::arch::asm;

#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base: u64,
}

static mut GDT: [GdtEntry; 3] = [
    // null
    GdtEntry { limit_low: 0, base_low: 0, base_mid: 0, access: 0, granularity: 0, base_high: 0 },
    // code: base 0, limit fffff, exec/read, 64-bit
    GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x9A, granularity: 0xAF, base_high: 0 },
    // data
    GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x92, granularity: 0xCF, base_high: 0 },
];

pub unsafe fn init() {
    let ptr = GdtPtr {
        limit: (core::mem::size_of::<[GdtEntry; 3]>() - 1) as u16,
        base: &GDT as *const _ as u64,
    };
    asm!("lgdt [{0}]", in(reg) &ptr);
    // reload data segments, keep CS for now (need far jump, todo)
    asm!(
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax",
        out("ax") _,
    );
}
