_start:
push RBP
call main
mov rdi , rax  ; User Defined Inline Assembly
mov rax , 60  ; User Defined Inline Assembly
syscall  ; User Defined Inline Assembly
pop RBP
ret
add_two:
push RBP
push RDI
mov EAX, EDI
add EAX, 2
mov EAX, EAX
pop RDI
pop RBP
ret
main:
push RBP
mov EDI, 2
call add_two
mov EAX, EAX
pop RBP
ret
