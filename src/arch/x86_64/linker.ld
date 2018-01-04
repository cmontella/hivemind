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
        *(.text)
    }
}