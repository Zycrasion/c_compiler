[[global _start]];

void exit(int exit_code)
{
    [[mov rax, 60]];
    [[syscall]];
    return;
}

// This is a wrapper for normal C files
void _start()
{
    exit(main());
    return;
}