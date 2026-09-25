# Changelog

## v0.1-dev (unreleased -> 2026-10-11)

- Boot Limine UEFI in QEMU, banner over serial
- GDT (Limine layout) + IDT (`#DE`, `#BP` returns, `#DF`, `#GP`, `#PF`), `int3` self-test
- Serial `print!`, panic to serial, `\r\n` fix
- Bounded line input (128, truncate, backspace, ignore ESC)
- Shell: `help`, `echo`, `about`, `version`, `mem`, `clear`, `halt`, `reboot` (8042, QEMU-only)
- `mem`: honest dump of Limine memmap, `checked_add` usable total, null guards
- `scripts/build.py` (ESP via `fat:rw:`), `scripts/run.py`, `scripts/smoke.py` (banner + commands)
- Linker: 4 KiB align, requests in writable `.data`, `relocation-model=static`

Earlier (2025):

- 2025-06-05: freestanding stub
- 2025-06-13: first Limine request (rev 0, wrong section)
- 2025-07-25: serial prints, banner (divisor + rev 3 fix)
- 2025-09-05: fix ELF PHDR sharing page
- 2025-10-03: GDT/IDT stubs
- 2025-11-14: console input (overflow bug)
- 2025-12-05: shell, empty-line guard

2026:

- 2026-02-13: mem rough dump (request ID was off by a nibble, fixed)
- 2026-03-06: honest mem reporting
- 2026-04-17: `clear`, `halt`
- 2026-05-15: `reboot`
- 2026-07-10/24: smoke tests
- 2026-08-14: repro build
- 2026-09-04: warning cleanup, boundary fixes
