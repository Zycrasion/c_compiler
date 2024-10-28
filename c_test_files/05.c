int main()
{
    int* size = 4;
    int a = 1;
    int b = 2;
    int c = 3;
    int d = 4;

    // Do something really stupid
    int* a_ptr = &a;
    int* b_ptr = a_ptr - size;
    int* c_ptr = b_ptr - size;
    int* d_ptr = c_ptr - size;

    return *d_ptr;
}