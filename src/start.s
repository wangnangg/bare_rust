.globl _start
.extern LD_STACK_PTR

.section ".text.boot"

_start:
ldr     x30, =LD_STACK_PTR
    mov     sp, x30
    bl      not_main
1:  wfe
    b       1b
