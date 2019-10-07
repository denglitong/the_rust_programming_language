// unsafe Rust 之所以存在，是因为静态分析本质上是保守的
// 可以使用不安全代码告诉编译器，"相信我，我知道我在干什么"
// 另一个 unsafe Rust 存在的原因是：底层计算机硬件固有的不安全性
// 如果 Rust 不允许进行不安全操作那么有些任务根据完成不了

// unsafe Rust 体统四个能力：
// 1.解引用裸指针
// 2.调用不安全的函数或方法
// 3.访问或修改静态变量
// 4.实现不安全 trait
// unsafe Rust 并不会关闭借用检查或禁用任何其他 Rust 安全检查，
// unsafe 关键字只是提供了那四个不会被编译器检查内存安全的功能，你仍然能在不安全快中获得某种程度的安全

// 保持 unsafe 快尽可能小

use crate::trates::{
    add_one, do_twice, Animal, Counter, Dog, Human, Kilometers, MyIterator, Pilot, Point, Wizard,
    Wrapper,
};
use core::slice;
use std::prelude::v1::Iterator;

// "C"部分 定义了外部函数所使用的 应用程序接口（application binary interface, ABI）
// ---ABI定义了如何在汇编语言层面调用此函数，"C" ABI 是最常见的
extern "C" {
    // 列出希望能调用的另一个语言中的外部函数的签名
    fn abs(input: i32) -> i32;
}

// Rust 中的 静态（static）变量，即全局变量
// 静态变量声明需注明类型，且只能存储 'static lifetimes 的引用
// 常量和静态变量的微妙区别是 static variable 是有固定内存地址的，
// 而常量是在被用到的时候复制其数据
static HELLO_WORLD: &str = "Hello, world!";

// 静态变量是可变的，不过访问和修改可变静态变量都是 unsafe 的
// 在多线程下访问静态变量会存在数据竞争问题
// 此处推荐优先使用 Met<T> 智能指针
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

mod trates;

fn main() {
    let mut num = 5;

    // 解引用裸指针，这里的 * 是类型名称的一部分
    // 裸指针可以：
    // 1.允许忽略借用规则，可同时拥有不可变和可变的指针，或有多个指向相同位置的可变指针
    // 2.不保证指向有效的内存
    // 3.允许为空
    // 4.不能实现任何自动清理功能
    // 可以直接在安全代码创建裸指针，但要解引用使用需要在 unsafe 快里边
    let r1 = &num as *const i32; // 不可变的裸指针，as 是强转
    let r2 = &mut num as *mut i32; // 可变的裸指针

    let address = 0x012345usize;
    // 创建执行任意内存地址的裸指针
    let r = address as *const i32;

    // 裸指针一个主要的应用场景是调用 C 代码接口，
    // 另一个场景是构建借用检查器无法理解的安全抽象
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x012345usize;
    // 我们并拥有这个任意地址的内存，也不能保证这段代码创建的 slice 包含有效的 i32 值
    let r = address as *mut i32;
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 100) };
    // println!("{}, {:?}", slice.len(), slice.get(0)); // segmentation fault: 11

    // 使用 extern 函数调用外部代码
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    let msg = "something";
    let c = Context(msg);
    let content = parse_context(c);
    if let Err(msg) = content {
        println!("msg: {}", msg);
    }

    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;

    let mut counter = Counter::new();
    while let Some(v) = counter.next() {
        println!("{:?}", v);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    // 调用 Human 在 Pilot trait 实现的 fly 函数，
    // 因为 fly 方法获取了一个 self 参数，可以根据 self 的类型计算出应该使用哪一个 trait实现
    // 然而如果是关联函数，则没有类型绑定，这是需要使用 完全限定语法（fully qualified syntax） 显式指定
    Pilot::fly(&person);
    // 调用 Human 在 Wizard trait 实现的 fly 函数
    Wizard::fly(&person);
    // 调用 struct Human 自身定义的 fly 方法
    person.fly();

    // 调用的是绑定在 struct 上的方法
    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name());
    // 完全限定语法 <Type as Trait>::function()，调用 Dog 在 Animal 上的实现
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let x: Kilometers = 32;
    println!("Kilometers is {:?}", x);

    //    let guess: u32 = match "34".trim().parse() {
    //        Ok(num) => num,
    //        Err(_) => continue,
    //    };

    // 动态大小类型和 size trait
    // str 本身是一个动态大小类型，直到运行是我们都不知道字符串有多长
    // let s1: str = "Hello there!"; // 这行代码不能编译
    // 为了解决这个问题，我们需要使用 &str 而不是 str，slice数据存储了开始位置和slice的长度
    // 虽然 &T 是一个存储了 T 所在的内存位置的单个值，&str 则是两个值：str的地址和其长度
    // 这样，&str 就有了一个在编译时可以知道的大小：它是 usize 长度的2倍，我们总是知道 &str 的大小的
    // Rust 中动态大小类型的常规做法是：必须将动态大小类型的值至于某种指针之后

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    // 闭包风格
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // 函数风格
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list_of_strings: {:?}", list_of_strings);

    let list_of_statuses: Vec<Status> = (0u32..3).map(Status::Value).collect();
    println!("list_of_statuses: {:?}", list_of_statuses);
}

#[derive(Debug)]
enum Status {
    // 这些项使用 () 作为初始化语法，这看起来就像函数调用，同时它们确实被实现为返回由参赛构造的实例的函数
    // 它们被称为实现了闭包trait的函数指针
    Value(u32),
    Stop,
}

// 匿名生命周期
struct StrWrap<'a>(&'a str);

// '_ 表示此处使用省略的生命周期
fn foo(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}

//impl fmt::Debug for StrWrap<'_> {
//}

//fn foo<'a>(string: &'a str) -> StrWrap<'a> {
//    StrWrap(string)
//}

// trait 对象必须遵守的生命周期规则：
// 1.trait 对象的默认声明周期是 'static
// 2.如果有 &'a X 和 &'a mut X，则默认 lifetimes 是 'a
// 3.如果只有 T: 'a，则默认生命周期是 'a
// 4.如果有多个类似 T: 'a, 则没有默认声明周期，必须明确指定
trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

struct Ref1<'a, T>(&'a T);
// 给泛型增加 lifetimes bound 来指定 T中的任何音乐至少与 'a 存活的一样久
// 不过在 Rust 1.31 版本后编译器已经能自动推断
struct Ref2<'a, T: 'a>(&'a T);
struct StaticRef<T: 'static>(&'static T);

struct Context<'s>(&'s str);

// https://kaisery.github.io/trpl-zh-cn/ch19-02-advanced-lifetimes.html
// https://github.com/rust-lang/book/issues/1969
// 在 Rust 1.31 版本之前，你需要 声明周期子类型（lifetime subtyping）来告诉编译器 'c, 's 哪个生命周期更长一些
// 在 Rust 1.31 版本之后，编译器会自动推断，如果生命周期不对则会编译报错
//struct Parser<'c, 's: 'c> {
struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

//impl<'c, 's: 'c> Parser<'c, 's> {
impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

// 不安全 trait
unsafe trait Foo {}

// 实现不安全 trait
unsafe impl Foo for i32 {}

// 不安全函数，必须在 unsafe 快中才能通过编译
unsafe fn dangerous() {
    println!("Something dangerous...")
}
