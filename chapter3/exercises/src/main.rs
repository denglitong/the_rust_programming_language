fn main() {
    println!("{}", fahrenheit_2_celsius(36.5));
    println!("{}", celsius_2_fahrenheit(2.5));
    for i in 1..10 {
        println!("{} => {}", i, fibonacci(i));
    }
}

// C＝(F－32)×5／9 F--华氏温度，C--摄氏温度
fn fahrenheit_2_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// F＝(C×9／5)＋32
fn celsius_2_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

// 1 1 2 3 5 8 13 ...
fn fibonacci(n: u128) -> u128 {
    if n <= 2 {
        return 1;
    }

    let mut a: u128 = 1;
    let mut b: u128 = 1;
    let mut t:u128 = 0;

    for i in 3..n+1 {
        t = b;
        b = a + b;
        a = t;
    }

    b
}