// mem rough dump - just print what limine gives

use crate::{limine, println};

pub fn dump() {
    // bootloader writes response from outside Rust, use volatile
    let resp_addr = unsafe { core::ptr::read_volatile(&limine::MEMMAP_REQUEST.response) };
    if resp_addr == 0 {
        println!("mem: no memmap response");
        return;
    }
    let resp = unsafe { &*(resp_addr as *const limine::MemmapResponse) };
    let count = resp.entry_count;
    println!("memmap entries: {}", count);
    let entries_ptr = resp.entries as *const u64;
    let mut i = 0;
    while i < count && i < 32 {
        let entry_addr = unsafe { *((entries_ptr as *const u64).add(i as usize)) };
        let e = unsafe { &*(entry_addr as *const limine::MemmapEntry) };
        // rough, assume kind numbers mean something
        println!("  {:#x} len {:#x} type {}", e.base, e.length, e.kind);
        i += 1;
    }
    // naive total, may overflow
    let mut total: u64 = 0;
    let mut j = 0;
    while j < count {
        let entry_addr = unsafe { *((entries_ptr as *const u64).add(j as usize)) };
        let e = unsafe { &*(entry_addr as *const limine::MemmapEntry) };
        if e.kind == 0 {
            total += e.length;
        }
        j += 1;
    }
    println!("usable (rough): {:#x} bytes", total);
}
