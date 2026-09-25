# BlackBoldOS

Minimal freestanding OS in Rust. Boots via Limine (UEFI) in QEMU, prints banner over COM1 serial, runs a tiny interactive shell.

Verified in QEMU only. No real hardware tested.

## What works (v0.1)

- `cargo build` produces `target/x86_64-unknown-none/debug/blackboldos` (ELF, higher half `0xffffffff80000000`)
- Limine 12.x UEFI `BOOTX64.EFI` loads the kernel (`protocol: limine`, `kernel_path: boot():/boot/blackboldos.elf`)
- Serial COM1 115200 8N1: `println!`, panic to serial, `\r\n` handling
- GDT (matches Limine layout, selectors `0x28`/`0x30`) + IDT with diagnostic handlers for `#DE`, `#BP` (returns), `#DF`, `#GP`, `#PF`. Boot does `int3` to prove IDT, prints `idt ok`.
- Bounded line input (128 bytes, truncate, backspace `0x08`/`0x7F`, ignore ESC)
- Shell: `help`, `echo`, `about`, `version`, `mem`, `clear`, `halt`, `reboot`
- `mem` dumps Limine memmap (base/len/type) and sums `usable` with `checked_add`, honest about types
- Smoke test builds, boots, sends commands slowly over stdio serial, greps log

## Build

Needs Rust stable + `x86_64-unknown-none` target:

```
rustup target add x86_64-unknown-none
cargo build
python scripts/build.py
```

`scripts/build.py` downloads Limine binary tar (≈2 MB) to `limine-bin/` on first run, copies `BOOTX64.EFI`, kernel ELF, and `limine.conf` into `esp/`:

```
esp/EFI/BOOT/BOOTX64.EFI
esp/boot/blackboldos.elf
esp/boot/limine/limine.conf
```

QEMU boots `esp/` via its `fat:rw:` driver, no ISO tools needed on Windows.

## Run

Needs QEMU + OVMF (`edk2-x86_64-code.fd` ships with QEMU on Windows, or `/usr/share/OVMF` on Linux).

Interactive:

```
python scripts/run.py
```

Manual QEMU (Windows example):

```
"C:\Program Files\qemu\qemu-system-x86_64.exe" -M q35 -drive if=pflash,unit=0,format=raw,file="C:\Program Files\qemu\share\edk2-x86_64-code.fd",readonly=on -drive format=raw,file=fat:rw:esp -serial stdio -display none -m 512
```

Type `help`. `halt` stops guest (QEMU keeps running, close it). `reboot` tries `outb(0x64,0xFE)`.

## Test

```
python scripts/smoke.py
```

Sends `help`, `echo`, `version`, `about`, `mem`, empty line, 150-char line (truncate test) char-by-char (≈15 ms) to avoid UART overrun, checks `serial.log` for banner + outputs. Exits 0 on `SMOKE PASS`.

Covers: banner `BlackBoldOS`, `boot ok`, `idt ok`, help, echo, version `v0.1`, memmap + usable total, empty/long line survival. Does NOT test `clear` (wipes log), `halt`/`reboot` (stops guest).

## Shell

```
> help
commands: help, echo, about, version, mem, clear, halt, reboot
> echo hi
hi
> version
BlackBoldOS v0.1-dev
> mem
memmap: 34 entries from bootloader
  0x0 len 0xa0000 usable
  ...
usable total: 0x... (... KiB)
note: only type 0 counted as usable
```

Empty line is ignored. Lines >128 are truncated. Backspace at col 0 does nothing.

## Boot path

OVMF → `EFI/BOOT/BOOTX64.EFI` (Limine) → `boot/blackboldos.elf` (`_start`) → `serial::init` → `gdt::init` → `idt::init` → `int3` test → banner → `shell::run`.

Linker aligns `.text`/`.rodata`/`.data`/`.bss` to 4 KiB pages or Limine panics (`PHDRs sharing page`). Requests (base rev 3, memmap) live in writable `.data` or bootloader can't fill `response`. `relocation-model=static` + `code-model=kernel` avoids dynamic PHDRs.

## Limitations, honest

- QEMU q35 + OVMF only. No BIOS, no real HW, no other arch.
- No filesystem, networking, GUI, allocator, multitasking, userspace.
- `reboot` is 8042 only, tested on QEMU. Real HW likely needs ACPI.
- `mem` trusts bootloader types; counts only type 0 as usable.
- Serial needs slow send (≈15 ms/char) or QEMU drops bytes under burst.
- `clear` is ANSI only, `halt` is `cli;hlt` loop.
- Base revision 3 (not latest 6). Works on Limine 12.9.3, may need bump later.

See `TODO.md`, `CHANGELOG.md`.
