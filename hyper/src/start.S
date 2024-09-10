.option norvc

.section .text.start

.global _start
_start:
.option push
.option norelax
    la gp, _global_pointer
.option pop
    la sp, _stack_end
    csrw sstatus, zero
    csrw sie, zero

    la t1, hyper_init
    la ra, 1f
    jr t1

    j 1f
1:
    wfi
    j 1b