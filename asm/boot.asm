global start
extern kernel_main

section .text
bits 32

start:
    mov esp, stack_top
    call check_multiboot
    call check_cpuid
    call kernel_main

check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
    .no_multiboot:
        mov al, '0'
        jmp error

check_cpuid:
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21
    push eax
    popfd
    pushfd
    pop eax
    push ecx
    popfd
    cmp eax, ecx
    je .no_cpuid
    ret
    .no_cpuid:
        mov al, '1'
        jmp error

%macro WRITE_CHAR 2
    mov byte [%1], %2
    mov byte [%1 + 1], 0x4f
%endmacro

error:
    mov cl, 0x4f
    WRITE_CHAR 0xb8000, 'E'
    WRITE_CHAR 0xb8002, 'R'
    WRITE_CHAR 0xb8004, 'R'
    WRITE_CHAR 0xb8006, 'O'
    WRITE_CHAR 0xb8008, 'R'
    WRITE_CHAR 0xb800a, ':'
    WRITE_CHAR 0xb800c, ' '
    WRITE_CHAR 0xb800e, al
    hlt

section .bss
align 4096
stack_bottom:
    resb 4096 * 128
stack_top: