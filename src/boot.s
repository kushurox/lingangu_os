.section .init

.option norvc


.global cringe


.section .kernel_entry
cringe:
    csrw satp, x0

    la sp, stack_top
    la gp, global_pointer
    la t0, kmain

    csrw mepc, t0
    csrr t1, mhartid

    beq t1, x0, A

    B:
        tail do_nothing
    A:
        tail kmain
