.global _start
.extern LD_STACK_PTR

.section .text.boot

_start:
    mrs x1, mpidr_el1
    and x1, x1, #3
    cbnz x1, 1f
    ldr x30, =LD_STACK_PTR
    mov sp, x30
    bl main
1:  wfe
    b       1b
