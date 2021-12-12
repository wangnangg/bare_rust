
SECTIONS
{
    . = 0x80000;
    .text : { *(.text.boot) }
    .rodata : { *(.rodata*)}
    .data : { *(.data*)}

    . = ALIGN(8);
    LD_STACK_PTR = .;

   /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
