[bits 64]
section .text
global _start
_start:
push RBP
call main
mov rdi , rax  ; User Defined Inline Assembly
mov rax , 60  ; User Defined Inline Assembly
syscall  ; User Defined Inline Assembly

main:
push RBP
mov ESI, 124
mov EDX, ESI
mov ESI, 2
mov ECX, ESI
mov EAX, EDX
pop RBP
ret
