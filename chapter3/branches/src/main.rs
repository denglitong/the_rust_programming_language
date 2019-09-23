use std::intrinsics::prefetch_read_instruction;

fn main() {
    let number = 7;

    // if 表达式中与条件关联的代码块有时被叫做 arms
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 代码中的条件必须是 bool 值， expected bool, found integer
    //if number {
    //    println!("number was three")
    //}

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // 注意代码块不同分支的返回值得一致
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // expected integer, found &str
    //let number = if condition { 5 } else { "six" };
    //println!("The value of number is: {}", number);
}
