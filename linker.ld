ENTRY(_start)
SECTIONS {
    /* limine wants higher half */
    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.data .data.*)
        KEEP(*(.requests_start_marker))
        KEEP(*(.requests))
        KEEP(*(.requests_end_marker))
        KEEP(*(.limine_reqs))
    }
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }
}
