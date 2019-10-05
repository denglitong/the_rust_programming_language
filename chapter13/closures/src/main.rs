use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // rust 闭包的语法有点像 ruby 的闭包语法
    // 只在有需要的时候才执行代码
    // 闭包没有参数和返回值的类型注明，闭包通常很短并只存在于较短的上下文中，这样编译能可靠的推断参数和返回值的类型
    // let expensive_closure = |num: u32| -> u32 {
    //    let expensive_closure = |num| {
    //        println!("calculating slowly...");
    //        thread::sleep(Duration::from_secs(2));
    //        num
    //    };
    // lazy initialization
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }

    println!("{}", expensive_result.value(1));
    println!("{}", expensive_result.value(2));

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    println!("{}", expensive_result.value("hello, world"));
}

// Fn, FnMut, FnOnce
// 函数也都实现了这三个 Fn trait，如果不需要捕获环境中的值，则可以使用实现了 Fn trait 的函数而不是闭包
// 闭包可以通过3种方式捕获其环境变量：它们对应函数的三种获取参数的方式：获取所有权、可变借用、不可变借用
// 这3中方式被编码为对应的 Fn trait:
// FnOnce: 消费从作用域中捕获的变量，必须获取其所有权并在定义闭包时将其移动进闭包，Once 表示了不能多次获取，
// 实际上它只能被调用一次（所以的闭包都可以被调用至少一次，所以所以闭包都实现了 FnOnce）；
// FnMut 获取可变的借用值，所以可以改变其上下文中的值
// Fn 从其环境获取不可变的借用值
struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    value: HashMap<U, Option<U>>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value.get(&arg) {
            Some(v) => v.unwrap(),
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, Some(v));
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    // generate_workout(simulated_user_specified_value, simulated_random_number);

    // 闭包可以捕获其环境变量
    let x = vec![1, 2, 3];
    // equal_to_x 闭包不可变的借用了 x，所以 equal_to_x 具有 Fn trait，引用闭包只需要读取 x 的值
    // 如果你希望闭包强制的获取使用的环境变量的ownership，则可以在参数列表前使用 move (针对非 Copy 类型)
    let equal_to_x = move |z| z == x;

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    //println!("can't use x here: {:?}", x);

    // 函数不能捕获环境变量
    //    fn equal_to_x_fn(z: i32) -> bool {
    //        // can't capture dynamic environment in a fn item
    //        z == x
    //    }
    //    assert!(equal_to_x_fn(y));

    // iterator
    let mut v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // iter() 生成一个不可变引用的迭代器

    //let v1_into_iter = v1.into_iter(); // 生成一个获取所有权并返回拥有所有权的迭代器
    //let v1_iter_mut = v1.iter_mut(); // 对可变引用返回迭代器

    // for 循环中 无需使迭代器变为 mut 是因为 move 进去后后台使其 mut 了
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // println!("v1 iter {:?}", v1_iter); // value borrowed here after move

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
