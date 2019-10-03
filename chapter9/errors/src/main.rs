// Rust 中的错误：可恢复错误（recoverable）和 不可恢复错误（unrecoverable）
// 可恢复错误通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件
// 不可恢复错误通常是 bug 的同义词，比如尝试访问超过数组结尾的位置

// 大部分语言并不区分这两类错误，并采用类似一次的方式统一处理他们
// 但 Rust 并不是这样，相反 Rust 并没有异常
// 对于可恢复错误，Rust有Result<T, E>
// 在遇到不可恢复时停止程序执行：panic!

use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{fs, io};

//enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}

//main 函数也是可以返回 Result<T, E> 的
// Box<dyn Error> 被称为 trait 对象，目前可以理解为 任何类型的错误
//fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    //panic!("crash and burn");

    let v = vec![1, 2, 3];
    // C语言中如果不判断索引是否溢出则可能导致 缓冲区溢出（buffer overread） 的问题
    // 即可返回并不属于 vector 内存的内容，这可能会导致安全漏洞
    //v[99];

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            _ => panic!("There was a problem opening the file: {:?}", error),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 失败时 panic 的简写：unwrap 和 expect
    // unwrap 类似于上面的 match 语义，如果 Result 值是 Ok 则返回 Ok 中的值
    // 如果 Result 是成员 Err，unwrap 会为我们调用 panic!
    //let f = File::open("world.txt").unwrap();
    // expect 类似于 unwrap 的方法，允许我们选择 panic! 的错误信息，
    // 一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源
    //let f = File::open("world.txt").expect("Failed to open world.txt");

    // ? 只能用于返回类型为 Result 的函数（在非 Result 返回类型的函数中不能使用），
    // 因为它被定义为与 match 表达式有着完全相同的工作方式
    // 在不返回 Result 的函数中，当调用其他返回 Result 的函数时，需要使用 match 或 Result 的方法之一来处理
    // 而不能用 ? 将潜在的错误传播给代码调用方
    //let f = File::open("hello.txt")?; // cannot use the `?` operator in a function that returns `()`

    //let username = read_username_from_file3()?;
    let username = read_username_from_file3();
    //username.expect("Problem read username from file");

    // main return Result<T, E>
    // Ok(())

    // panic! or not panic!
    // Rust 的类型系统可以进行很多检查
    // 如果函数有一个特定类型的参数，可以在知晓编译器已经确保其拥有一个有效值的前提下进行你的代码逻辑，
    // 例如，如果你使用了一个不同于 Option 的类型，而且程序期望它是有值的并且不是控制，
    // 你的代码无需处理 Some 和 None 这2种情况，它只有一种情况就是绝对有值
    // 尝试向函数传递控制的代码根本不能编译，所以你的函数在运行时没有必要判空【em...比Java好很多啊】

    // panic! 代表一个程序无法处理的状态
    // Result 枚举代表操作可能会在一种可能可以恢复的情况下失败
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    // 熟悉的 getter
    pub fn value(&self) -> i32 {
        self.value
    }
}

// 标准库加持，更短的写法：
// 读取一个字符串是相当常见的操作，所以 Rust 提供了名为 fs::read_to_string 的函数 【赞 Rust 的工程学！】
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

// 使用链式操作精简代码，更与众不同、符合工程学~
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("username.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// 传播错误（propagating），将错误的处理交给上下文中的调用方
// 这种传播错误的模式在 Rust 是如此的常见，以至于有一个更简便的专用语法：?
fn read_username_from_file2() -> Result<String, io::Error> {
    // 如果 Ok(t) 则将值赋给变量，否则如果有 Err(e) 则立即返回 Err(e) 【比Go爽很多】
    // 这里的 ? 与 match 所做的工作有一点不同：? 所使用的 错误被传递给了 from 函数，
    // 这是定义在标准库中的 From trait 中，其用来将错误从一种类型转换为另一种类型，
    // 这当在一个函数返回一个错误类型来代表所有可能失败的方式时很有用，
    // 只要每一个错误类型都实现了 from 函数来定义如何将其转换为返回的错误类型
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    // ? 消除了大量样板代码并使得函数的实现更简单，链式操作也可以进一步精简代码
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
