// This is a wrapper for normal C files

void _start()
{
    main();
    [[mov rdi, rax]];
    [[mov rax, 60]];
    [[syscall]];
}