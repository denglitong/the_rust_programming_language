fn main() {
    // loop {
    //     println!("again!");
    // }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // break 用来停止循环，可以在 break 后面带一个表达式作为循环的返回值
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!")
}
