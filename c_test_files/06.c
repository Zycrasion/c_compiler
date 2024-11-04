int main()
{
    char* hello_world = "Hello World!";
    int* string_length = alloc(4);
    *string_length = 13;

    println(hello_world, *string_length);

    println("Hello Universe!", 16);

    return 0;
}