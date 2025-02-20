INPUT(boot.o)
MEMORY {
    DRAM : ORIGIN = 0x80000000, LENGTH = 512K
}
SECTIONS {
    /* . = 0x80000000; */
    .text : ALIGN(4K){
        KEEP(*(.kernel_entry))
            *(.kernel_entry)
            *(.text)
            *(.text*)
    } > DRAM

    .bss : ALIGN(4K){
        *(.bss);
        PROVIDE(stack_top = .);
        . += 4096;
        PROVIDE(global_pointer = .);
    } > DRAM

    .data : ALIGN(4K){
        *(.data)
        *(.data*)
    } > DRAM

    .rodata : ALIGN(4K){
        *(.rodata)
        *(.rodata*)
    } > DRAM
}
