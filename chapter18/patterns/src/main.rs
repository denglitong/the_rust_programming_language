use crate::Color::{Hsv, Rgb};

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!")
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // Rust 中的模式就存在于 let 语句中
    // 将元组 (1,2,3) 与 (x,y,z) 匹配，匹配之后将对应的字面量绑定到变量上
    let (x, y, z) = (1, 2, 3);
    // let (x, y) = (1, 2, 3); // 不匹配而报错

    // 这个也是 Rust 中的的模式，
    // let PATTERN = EXPRESSION;
    // 变量名不过是形式特别朴素的模式，将表达式与模式比较，并为任何找到的名称赋值
    // 这个模式实际上等于"将任何值绑定到变量 x"
    let x = 5;

    let point = (3, 5);
    print_coordinates(&point);

    // let Some(x) = Option::Some(32);
    if let x = 5 {
        println!("{}", x);
    };

    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 5;
    match x {
        // 范围只需要数字或char值
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'r';
    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("Something else"),
    }

    let p = Point { x: 0, y: 7 };

    // 解构 struct 为单独的变量
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // 字段名简写语法
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(color) => match color {
            Color::Rgb(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b);
            }
            Color::Hsv(h, s, v) => {
                println!(
                    "Change the color to hue {}, saturation {}, and lightness {}",
                    h, s, v
                );
            }
            _ => (),
        },
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    // 通过模式解构是一个方便利用部分值片段的手段
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }

    // 使用 _ 来忽略未使用的变量
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello"));
    if let Some(_s) = s {
        // move here
        println!("found a string: {}", _s);
    }
    // println!("{:?}", s); // can not use after move in if let or match pattern

    let origin = Point3 { x: 0, y: 0, z: 0 };
    match origin {
        // 使用 .. 来忽略剩余的字段
        Point3 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // 使用 .. 需要是没有歧义的才可以
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // 匹配守卫 match guard 是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支
    let num = Some(4);
    match num {
        // 无法在模式中表达 if x < 5 的条件，所以匹配守卫提供了表现次逻辑的能力
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => print!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // n 是匹配模式中的，y 是外部变量，这里没有引入匹配变量而覆盖y，这正是匹配守卫发挥的作用
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        // (4|5|6) if y => ..
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ 绑定
    // @ 运算符允许我们在创建一个存放值的变量的同时测试其值是否匹配模式

    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello {
            id: id_variable @ 3...7,
        } => {
            println!("Found an id in range: {}", id_variable);
        }
        Message2::Hello { id: 10...12 } => {
            println!("Found an id in another range");
        }
        Message2::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}

enum Message2 {
    Hello { id: i32 },
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

// _ 模式可以用在函数参数中，用于忽略某个参数
// 大部分情况当你不再需要特定函数参数时，最好修改签名不在包含无用的参数
fn bar(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

// 解构枚举的模式需要对应枚举所定义的存储数据的方式
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

struct Point {
    x: i32,
    y: i32,
}

// 模式有2种形式：refutable（可反驳的）和 irrefutable（不可反驳的）
// 能匹配任何传递的可能值的模式被称为是 不可反驳的 irrefutable，比如 let x = 5; 语句中的 x 模式
// 对某些可能的值进行匹配会失败的模式被称为是 可反驳的 refutable，比如 if let Some(x) = a_value 表达式中的 Some(x)

// let语句，函数参数，for循环 只能接受不可反驳的pattern
// if let, while let 被限制为只能接受可反驳的模式

// 函数参数也是模式！
// x 部分就是一个模式！类似于前面的 let x = 5
fn foo(x: i32) {}

// &(x, y) 就是一个模式，匹配一个元组
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
