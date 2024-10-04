[bits 64]
section .text
global _start
_start:
push RBP
call main  ; Inline Assembly
mov EAX, 0
pop RBP
ret

main:
push RBP
mov ESI, 0
mov EDX, ESI
mov ESI, 2
mov EAX, EDX
pop RBP
ret
