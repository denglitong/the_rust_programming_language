// 枚举 enumerations，也称 enums
// Rust 的枚举与 F#,OCaml,Haskell这样的函数式编程语言中的
// 代数数据类型（algebraic data types） 最为相似

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:?}, six: {:?}", four, six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}, {:?}", home, loopback);

    // let home = IpAddr2::V4(String::from("127.0.0.1"));
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    // 标准库中的IP地址类型
    let home = Ipv4Addr::from_str("127.0.0.1");
    let loopback = Ipv6Addr::from_str("::1");
    println!("{:?}", home); // Ok(127.0.0.1)
    println!("{:?}", loopback); // Ok(::1)

    let quit = Message::Quit;
    quit.call();

    // Option 枚举和其相对于空值的优势，它编码了一个非常普遍的场景，即一个值要么有值要么没值
    // 从类型系统的角度来表达这个概念就意味着编译器需要检查是否处理了所有应该处理的情况
    // 这样就可以避免在其他编程语言中非常常见的bug
    // 编程语言的设计经常要考虑包含哪些功能，但考虑排查哪些功能也很重要（package设计也是类似）
    // Rust 并没有很多其他语言中有的空值（Null）功能，但空值尝试表达的概念仍然是有意义的：
    // 空值是一个因为某种原因目前无效或缺失的值
    // 问题不在于概念而在于具体的实现，Rust 没有空值，不过它缺失拥有一个可以编码存在或不存在概念的枚举：Option<T>
    // Java8 也有了 Option
    // 其实这里是 Option::Some(5); 不过 Option<T> 是如此被广泛应用于是它被包含在了 prelude 之中，即不需要显示引入作用域
    let some_number = Some(5);
    let some_string = Some("a string");
    // None 需要指定类型参数，因为没有了自动推断
    let absent_string: Option<i32> = None;
    // Options<T> 为什么就比空值要好呢？首先，Option<T> 和 T 是不同的类型，不能一起运算
    //let x: i8 = 5;
    //let y: Option<i8> = Some(5);
    //let sum = x + y; // mismatched types
    // 当 Rust 中拥有一个像 i8 这样类型的值时，编译器确保它总是一个有效的值，我们可以放心使用而不需做空值检查
    // 只有当使用 Option<i8> 或者任何用到的类型的时候需要担心可能没有值，而编译器会确保我们在使用之前处理了为空的情况
    // 换句话说，在对 Option<T> 进行 T 的运算之前必须将其转换为 T，
    // 通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空但实际上未空的情况
    // 只要一个值不少 Option<T> 类型，你就可以安全的认定它的值不为空，
    // 这是 Rust 的一个经过深思熟虑的设计决策，来限制空值的泛滥以增加 Rust 代码的安全性
    println!("{}", some_string.is_some());
    println!("{}", some_string.is_none());
    println!("{}", absent_string.is_none());
    let a = "a";
    //println!("{}", some_string.contains(&a));

    // match 模式匹配控制流，match 的力量来源于模式的表现力以及编译器检查，它确保了所有可能的情况都得到处理
    value_in_cents(Coin::Quarter);
    value_in_cents2(Coin2::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);

    // _ 通配符会匹配所有的值
    let some_u8_value = 0u8;
    match some_u8_value {
        //1 => println!("one"),
        3 => println!("three"),
        //5 => println!("five"),
        //7 => println!("seven"),
        _ => (), // () 就是 unit 值
    }

    // 不过 match 在只关心 一个 的场景中可能就有点啰嗦了，为此 Rust 提供了 if let
    let some_u8_value = Some(3u8);
    // if let 获取通过等号分隔的一个模式和一个表达式，它的工作方式与 match 相同，这里的表达式对应 match 模式第一个分支
    // if let 意味着编写更少代码，更少的缩进和更少的样板代码，然后这样会失去 match 强制要求的穷尽性检查
    // match 和 if let 之间的选择依赖特定的环境已经增加简洁度和失去穷尽性的权衡取舍
    // 可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值
    // 可以在 if let 中包含一个 else，这和 match 表达式中的 _ 效果相同
    if let Some(3) = some_u8_value {
        println!("three")
    }

    let coin = Coin2::Quarter(UsState::Alaska);
    let mut count = 0;
    match &coin {
        Coin2::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    let mut count = 0;
    if let Coin2::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("coin count: {}", count);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // 将 match 与枚举相结合在很多场景中都是有用的
    // match 匹配是穷尽的，如果只 match 枚举中的部分值并不能通过编译
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u32 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    // 一个枚举及其成员作为模式的 match表达式
    match coin {
        // 如果分支代码较短的话通常不适应大括号
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("Lucky quarter!");
            25
        }
    }
}

// 你可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体，甚至可以包含另一个枚举
// 我们可以像 struct 那样，在 enum 上通过 impl 来定义方法：（再次体现 Rust 中的 enum 是 代数数据类型 algebraic data type）
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: u32, y: i32 }, // 渐渐体现了 Rust 中的枚举的 代数数据类型（algebraic data type）
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

// 我们可以使用不同的结构体来表达上述 Message 枚举的意思，但是由于它们都有不同的类型，不能像使用 Message 枚举那样，
// 轻易的定义一个能够处理不同类型的结构体的函数，而枚举是单独一个类型就可以这么做
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32);

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
// 这里使用了2个字段的struct来表示Ip地址，我们可以使用一种更简洁的方式来表达相同的概念：
#[derive(Debug)]
enum IpAddr2 {
    // 仅仅使用枚举并将数据放进每一个枚举成员而不是将枚举作为结构体的一部分
    // 这样就不需要一个额外的结构体了
    // 新定义表明了 V4 V6 成员都关联了 String 值
    // 用枚举代替结构体还有另外一个优势：每个成员可以处理不同类型和数量的数据
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}
