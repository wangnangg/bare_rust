
SECTIONS
{
    . = 0x80000;
    .text : { *(.text.boot) }

    . = ALIGN(8);
    LD_STACK_PTR = .;

   /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
