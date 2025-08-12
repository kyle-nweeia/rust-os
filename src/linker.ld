. = 0x80000;

SECTIONS {
    .text : {
        *(.text*);
    }

    .rodata : {
        *(.rodata*);
    }
}
