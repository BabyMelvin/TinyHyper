    .section .text.init
    .global _trap_entry
    .balign 4
_trap_entry:
    sret