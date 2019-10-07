//
// Created by denglitong on 2019/10/6.
//
// gcc src/rust_hello.c -L ./target/debug/ -lrust_hello -o test

extern void call_from_c();
extern int add_one(int input);

int main() {
    call_from_c();
    int input = 4;
    int output = add_one(input);
    printf("%d +1 = %d\n", input, output);
    return 0;
}

