// 表示编译成动态库
#![crate_type = "dylib"]

// 从其他语言调用 Rust 函数，下面的例子如果编译为动态库并从 C 语言中链接，call_from_c 函数就能够在 C 代码中访问
// no_mangle 告诉 Rust 编译器不要擅自改动这个函数的函数名
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!")
}

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
