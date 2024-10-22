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
push RDI
mov EAX, EDI
pop RDI
pop RBP
ret
main:
push RBP
push RSI
mov EDI, 9
call some_func
mov ESI, EAX
mov EAX, ESI
pop RSI
pop RBP
ret
