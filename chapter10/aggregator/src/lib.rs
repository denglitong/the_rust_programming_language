use std::fmt::{Debug, Display};

// trait 类似其他语言中的 interface
// trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必须的行为的集合
// pub 可使得 trait 能被引用到
pub trait Summary {
    fn summarize_author(&self) -> String;
    // 方法签名
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    // trait 可以定义 default method（类似 Java8 里面的 interface 的 default 方法）
    // 默认方法可以调用 trait 中的其他方法，哪怕这些方法当前还没有默认实现（但是调用的实例对应的类型会必须实现这些方法）
    // 注意如果类型重载了默认方法，那在该方法里面无法再调用 trait 中的同名默认方法
    fn read_more(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 一旦实现了 trait，实例就可以使用调用非 trait 方法一样的方式调用 trait 方法
// 只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait
// 即不能为外部类型实现外部的 trait，如不能为 Vec<T> 实现 Display trait
// 这是因为 Display 和 Vec<T> 都定义于标准库中，并不位于本地作用域，
// 这个限制被称为 相干性（coherence）的程序熟悉的一部分，或者更具体的说是 孤儿规则（orphan rule）
// 这条规则确保了其他人编写的代码不会破坏你的代码，反之你也无法破坏他人编写的代码
impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    //    fn summarize(&self) -> String {
    //        format!("{}: {}", self.username, self.content)
    //    }

    // 重载一个方法和实现一个方法的语法一样
    //fn read_more(&self) -> String {}
}

// impl Trait 语法适用于短小的例子，它不过是一个较长形式的语法糖，这被称为 trait bound
// 看起来像 fn notify<T: Summary>(item: T) {}
// 有点类似于 Java 中的 <? implement interface>
// 通过 + 指定多个 trait，即 item: impl Summary + Display，
// fn notify<T: Summary + Display>(item: T) {}
// 这又有点类似 Java 中的逗号 <? implement interface1,interface2>
pub fn notify(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_two_args(item1: impl Summary, item2: impl Summary) {}
fn notify_same_impl_type<T: Summary>(item: T) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// 通过 where 简化复杂的 trait bound 代码，提示函数签名易读性
fn some_function_where<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    //
}

// 返回 trait
// 声明函数要返回某个实现了 Summary trait 的类型，但是不确定起具体的类型
// 不过当前这个函数只能返回单一类型的情况，如果想返回 Tweet 类型或者是 NewsArticle 类型，现在还不行（后续会讲
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // Pair 总是实现了 new 方法，即 new 方法对 T 是没有限制的
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    // Pair 只针对 实现了 Display + PartialOrd trait 的 T 才实现了 cmp_display 方法
    // 这杯称为 blanket implements
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
