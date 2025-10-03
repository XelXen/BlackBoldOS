// gdt - match limine layout so old selectors stay valid

use core::arch::asm;

#[repr(C, packed)]
#[derive(Clone, Copy)]
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

const NULL: GdtEntry = GdtEntry { limit_low: 0, base_low: 0, base_mid: 0, access: 0, granularity: 0, base_high: 0 };
// 16-bit code, base 0 limit ffff readable
const CODE16: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x9A, granularity: 0x0F, base_high: 0 };
const DATA16: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x92, granularity: 0x0F, base_high: 0 };
// 32-bit
const CODE32: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x9A, granularity: 0xCF, base_high: 0 };
const DATA32: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x92, granularity: 0xCF, base_high: 0 };
// 64-bit, limit irrelevant
const CODE64: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x9A, granularity: 0xAF, base_high: 0 };
const DATA64: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x92, granularity: 0xCF, base_high: 0 };

static mut GDT: [GdtEntry; 7] = [NULL, CODE16, DATA16, CODE32, DATA32, CODE64, DATA64];

pub unsafe fn init() {
    let ptr = GdtPtr {
        limit: (core::mem::size_of::<[GdtEntry; 7]>() - 1) as u16,
        base: &raw const GDT as *const _ as u64,
    };
    asm!("lgdt [{0}]", in(reg) &ptr);
    // reload data segs to 0x30 (64-bit data, index 6), keep CS 0x28 for now
    asm!(
        "mov ax, 0x30",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax",
        out("ax") _,
    );
}
