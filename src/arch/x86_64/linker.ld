ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* keep mutliboot header at the beginning */
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}