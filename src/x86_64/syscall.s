[BITS 64]

global arch_syscall
arch_syscall:
    mov rax, rdi
    mov rbx, rsi
    mov rsi, rcx

    syscall
    ret