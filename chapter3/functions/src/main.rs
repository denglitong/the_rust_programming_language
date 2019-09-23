fn main() {
    println!("Hello, world!");

    // Rust中的函数和变量名使用 snake case 规范风格
    another_function(5, 6);

    let _y = 6; // statement
    // let x = (let y = 6); // let 语句不返回值，不能用来赋值给其他变量
    // let y = 6; 中的 6 是一个表达式，它计算出的值是 6
    // 函数调用是一个表达式，宏调用也是一个表达式，
    // 用来创建新作用域代码块的大括号 {}，也是一个表达式
    let y = {
        let x = 3;
        // expression的结尾没有分号，如果加上分号就变成了statement，
        // 而statement并不会返回值
        x + 1
    };
    println!("The value of y is: {}", y);

    // let x = five();
    let x = plus_one(five());
    println!("The value of x is: {}", x)
}

// Rust 不关心函数定义的前后顺序，只有定义了就行
// 函数签名中，必须声明每个参数的类型，这样编译器就不需要自动推断
// Rust 是一门基于表达式（expression-based）的语言，这是一个不同于其他语言的重要区别
// 函数体由一系列语句和一个可选的结尾表达式构成
// 语句（statements）是执行一些操作但不返回值的指令
// 表达式（expressions）计算并产生一个值
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Rust 函数的返回值等同于函数体最后一个表达式的值（和 Ruby 相同）
// 可以使用 return 关键字和指定值提前返回，
// 但大部分函数是隐式的返回最后的表达式 expression
// （再次注意区分 statement 和 expression 的不同，一个执行操作但不返回，一个执行计算并返回一个值）
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // (); // 空元组 tuple let (x, y, z) = (1, 2, 3);
    x + 1
}