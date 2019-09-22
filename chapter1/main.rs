// Rust 是预编译静态类型（ahead-of-time complied）语言
fn main() {
    // println!() 调用了一个Rust宏（macro）
    // 如果调用函数，则是 println()
    // 当看到符号 ! 的时候，就意味着是宏调用而不是普通的函数调用
    println!("Hello, World!");
}