[[global _start]];

void exit(int exit_code)
{
    [[mov rax, 60]];
    [[syscall]];
    return;
}

void putchar(char* ptr)
{
    [[mov rax, 1]];
    [[mov rsi, rdi]];
    [[mov rdi, 0]];
    [[mov rdx, 1]];
    [[syscall]];
    return;
}

void print(char* ptr, int count)
{
    [[mov rax, 1]];
    [[mov rdx, rsi]];
    [[mov rsi, rdi]];
    [[mov rdi, 0]];
    [[syscall]];
    return;
}

void println(char* ptr, int count)
{
    print(ptr, count);
    char newline = 10;
    putchar(&newline);
}

// This is a wrapper for normal C files
void _start()
{
    exit(main());
    return;
}