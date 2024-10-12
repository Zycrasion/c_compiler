[bits 64]
section .text
global _start
_start:
push RBP
call main
mov rdi , rax  ; User Defined Inline Assembly
mov rax , 60  ; User Defined Inline Assembly
syscall  ; User Defined Inline Assembly
pop RBP
ret

some_func:
push RBP
mov EAX, 10
pop RBP
ret
main:
push RBP
call some_func
mov ESI, EAX
mov EAX, ESI
pop RBP
ret
