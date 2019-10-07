use core::fmt;
use std::error::Error;
use std::fmt::Formatter;
use std::intrinsics::write_bytes;
use std::ops::Add;

pub trait MyIterator {
    // 关联类型，Item 是一个占位类型，这个 trait 的实现者会指定 Item 的具体类型
    type Item;

    // 在未有具体实现之前，允许定义一个函数而不指定其具体可以处理的类型
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 这里不实现成 Iterator<T> 这种形式，是因为这样就可以多次实现这个 trait，
// 比如 impl Iterator<String> for Counter 这样，每次需改变泛型参数的具体类型，
// 接着当在调用是必须提供类型注解来表明希望使用 Iterator 的哪一个实现
// 通过关联类型，则无需标注类型，因为不能多次实现这个 trait
impl MyIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}

// 默认泛型类型参数和运算符重载

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// 实现 Add trait 来重载 + 运算符
impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 可以为泛型定义 默认类型参数，如果不指定 RHS 的具体类型，RHS 的类型将是默认的 Self 类型
// 也就是在其上实现 Add 的类型
trait MyAdd<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

// 为泛型类型参数指定具体类型
// 默认参数类型可以扩展类型而不破坏现有代码，并且在大部分用户都不需要的特定情况下可以进行自定义
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + other.0 * 1000)
    }
}

pub trait Pilot {
    fn fly(&self);
}
pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!")
    }
}

impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously");
    }
}

pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// 类似于为 trait 增加 trait bound，可以在一个 父trait（super trait） 下实现一个 trait
// 这是因为要实现的某个 trait 使用使用另一个 trait 的功能
pub trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

pub struct Point3 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point3 {}

// newtype模式 用以在外部type上实现外部trait
// 孤儿规则（orphan rule）限定了只有 trait或者type 在本地crate才能在该类型上实现该trait，
// 而 newtype模式 就是打破 orphan rule 的，
// newtype模式是 “封装隐藏了实现细节” 的一个轻量级方法

// 想为外部 type Vec<> 实现 外部trait fmt::Display，我们新建了一个包装类型
// 默认字段都是 private，这里需要设置为 pub
pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// type alias，同一个type，只是名称不同，类型别名的主要用途是减少重复
pub type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {}

// 从不返回的 never type
// ! 是 Rust 的特殊类型，被称为 empty type, 因为它没有值，也称 never type
// never type 的作用是在函数从不返回的时候充当返回值，从不返回值的函数被称为 发散函数（diverging functions）
//fn bar() -> ! {}

// 函数指针，fn 是一个 type 而不是 trait
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 闭包表现Wie trait，不能直接返回闭包，因为它们没有一个可返回的具体类型，编译器也就无法切确的知道该分配多少内存
//fn returns_closure() -> Fn(i32) -> i32 {
//    |x| x + 1
//}

// 这样是OK的，通过Box来保存一个动态指针编译器就知道内存如何分配到变量
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
