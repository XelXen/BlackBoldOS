# TODO / known issues (v0.1 done)

Done for v0.1: boot, banner, GDT/IDT, serial shell, mem, clear/halt/reboot (qemu), smoke.

Next ideas (not promised):
- bump base revision 3 -> 6, retest on newer Limine
- `uptime` via TSC? need calibrated frequency
- framebuffer info command (currently serial only)
- better `reboot` (ACPI), currently 8042 qemu-only
- persistent `serial.log` rotation in smoke

Known issues:
- serial drops bytes on burst, smoke sends slowly
- `clear` ANSI only
- no real HW tested
