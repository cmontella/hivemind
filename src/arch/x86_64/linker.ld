ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* keep mutliboot header at the beginning */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text .text.*)
    }

    .rodata : 
    {
        *(.rodata .rodata.*)
    }

    .data.rel.ro : 
    {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }
}