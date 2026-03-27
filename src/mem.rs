// honest mem reporting based on what limine actually gives

use crate::{limine, println};

fn kind_name(k: u64) -> &'static str {
    match k {
        0 => "usable",
        1 => "reserved",
        2 => "acpi_rec",
        3 => "acpi_nvs",
        4 => "bad",
        5 => "boot_rec",
        6 => "exec_mod",
        7 => "fb",
        8 => "res_map",
        _ => "unknown",
    }
}

pub fn dump() {
    let resp_addr = unsafe { core::ptr::read_volatile(&limine::MEMMAP_REQUEST.response) };
    if resp_addr == 0 {
        println!("mem: no memmap from bootloader");
        return;
    }
    let resp = unsafe { &*(resp_addr as *const limine::MemmapResponse) };
    let count = resp.entry_count;
    if count == 0 {
        println!("mem: empty memmap");
        return;
    }
    let entries_addr = unsafe { core::ptr::read_volatile(&resp.entries) };
    if entries_addr == 0 {
        println!("mem: null entries ptr");
        return;
    }
    println!("memmap: {} entries from bootloader", count);
    let entries_ptr = entries_addr as *const u64;
    // cap display to avoid spam, but count all for total
    let show = if count > 64 { 64 } else { count };
    let mut i = 0;
    while i < show {
        let entry_addr = unsafe { *(entries_ptr.add(i as usize)) };
        if entry_addr == 0 {
            println!("  [{}] null entry, skipping", i);
        } else {
            let e = unsafe { &*(entry_addr as *const limine::MemmapEntry) };
            println!(
                "  {:#x} len {:#x} {}",
                e.base,
                e.length,
                kind_name(e.kind)
            );
        }
        i += 1;
    }
    if count > show {
        println!("  ... {} more", count - show);
    }
    let mut total: u64 = 0;
    let mut overflow = false;
    let mut j = 0;
    while j < count {
        let entry_addr = unsafe { *(entries_ptr.add(j as usize)) };
        if entry_addr != 0 {
            let e = unsafe { &*(entry_addr as *const limine::MemmapEntry) };
            if e.kind == 0 {
                match total.checked_add(e.length) {
                    Some(v) => total = v,
                    None => {
                        overflow = true;
                        break;
                    }
                }
            }
        }
        j += 1;
    }
    if overflow {
        println!("usable total overflowed u64, report unreliable");
    } else {
        println!("usable total: {:#x} ({} KiB)", total, total / 1024);
    }
    println!("note: only type 0 counted as usable, others reserved by fw/bootloader");
}
