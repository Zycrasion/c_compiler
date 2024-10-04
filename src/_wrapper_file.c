// This is a wrapper for normal C files

int _start()
{
    [[call main]]; // Compiler directive for inline assembly
    return 0;
}