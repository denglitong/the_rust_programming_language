// 单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块或者是私有接口
// 而集成测试对于你的 库来说则完全是外部的，它们同其他外部代码一样调用你的代码或接口
// 而且每个测试都有可能会测试多个模块

// 单元测试与它们要测试的代码共同存放于 src 目录下相同的文件中，
// 规范是在每个文件中创建包含测试函数的 tests 模块，并使用 #[cfg(test)] 标注
// #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和执行测试代码

// Rust 的私有性规则允许你测试私有函数

#[cfg(test)]
mod tests {
    // 测试外部代码需要引入对应的 crate
    use crate::{add_two, greeting, prints_and_returns_10, Guess, Rectangle};

    #[test]
    fn exploration() {
        // assert_eq! 和 assert_ne! 宏在底层分别使用了 == 和 !=
        // 当断言失败，这些宏使用调试格式打印其参数
        // 这意味着被比较的值必须实现了 PartialEq 和 Debug
        // 所有的基本类型和大部分标准库类型都实现了这些 trait，对于自定义结构体和枚举
        // 需要实现这2个 trait 才能断言他们的值是否相等
        assert_eq!(4, add_two(2));

        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert_ne!(larger, smaller);
    }

    #[test]
    fn another() {
        // 每个测试都在一个新线程中执行，主线程发现测试线程异常了就将测试标记为失败
        // panic!("Make this test fail")
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        // assert! 宏接受一个 bool 参数，如果 bool 为 false，则 panic!
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            // 自定义的失败信息参数，可传递一个包含 {} 占位符的字符串以及需要放入占位符的值
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // 这里的 expected 字符串表示预期抛出的 panic 中的提示信息包含 expected 中指定的字符串
    // should_panic 注解检查测试函数是否如期抛出 panic，在抛出 panic 的时候通过，其他情况失败
    // 【类似于 JUNIT 中的 assertThrows(exception.class, fn)】
    fn guess_greater_than_100() {
        Guess::new(200);
    }

    // 将 Result<T, E> 用于测试
    #[test]
    // 可以通过返回 Result<T, E> 来判断测试的成功/失败的结果，不再通过 panic!
    // 为此不能再对这些函数使用 #[should_panic]，而是应该返回 Err!
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    // Rust 中的测试是互相独立的，默认多线程每个测试一个线程，此时注意对共享资源的读写可能造成的冲突
    // 使用 --test-threads=1 告诉程序不使用任何并行机制，此时测试就是串行的

    #[test]
    fn this_test_will_pass() {
        // 如果测试通过，在终端将看不到对应过程的 println! 输出，因为标准输出会被截获
        // 如果希望测试通过的也能看到中间过程的标准输出，可以增加 cargo test -- --nocapture 参数
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_fail() {
        // 如果测试失败，则会看到所有标准输出和其他错误信息
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    // 默认忽略，显示执行 cargo test -- --ignored
    fn expensive_test() {
        //
    }
}

#[derive(PartialEq, Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(i: i32) -> i32 {
    // i + 2
    internal_adder(i, 2)
}

// 私有函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    //format!("Hello {}!", name)
    format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 1 {
            panic!(
                "Guess value must be greater than 1 or equal to 1, got {}.",
                value
            );
        } else if value < 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            )
        }

        Guess { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
