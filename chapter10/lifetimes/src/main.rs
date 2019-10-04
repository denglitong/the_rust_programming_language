// 生命周期（lifetimes）是一种泛型 generics，不同于其他泛型帮助我们确保类型拥有期望的行为，
// 声明周期则有助于确保引用在我们需要的时候一直有效

// Rust 中的每一个引用都有其 lifetimes，即引用保持有效的 scope
// 像 type 能自动推断一样，lifetimes 也隐含能自动推断，
// 但是也像 type 在可能有多种的时候必须注明类型，
// lifetimes 在不同方式相关联的时候，也需要我们使用 generics lifetimes 参数来注明他们的关系
// 这样就能确保运行时实际使用的引用绝对是有效的（野指针的福音！，生命周期的主要目标就是避免悬垂指针）

use std::fmt::Display;

fn main() {
    let r;

    {
        let x = 5;
        r = &x; // borrowed value does not live long enough
    } // x 离开其 scope，内容会被释放

    //println!("r: {}", r);

    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    //    let string1 = String::from("long string is long");
    //    let result;
    //    {
    //        let string2 = String::from("xyz");
    //        // borrowed value does not live long enough
    //        result = longest(string1.as_str(), string2.as_str());
    //    } // 在这里 string2 失去起 lifetimes，
    //      // 而 longest 要求 result/string1/string2 需要在最小的交集中一起生效(而不是最长的昂或者是刚好引用到的那个)
    //    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept {
        part: first_sentence,
    };

    // 静态生命周期 是一种特殊的生命周期，其存活于整个程序期间，所有的字符串字面值都拥有 'static 生命周期
    // 字符串字面值是被直接编译到二进制文件中的，因此总是可用的
    let s: &'static str = "I have a static lifetime";
    println!("{}", s);

    let s1 = String::from("abcd");
    let s2 = "xyz";
    let ann = 3.14;
    let result = longest_with_an_announcement(s1.as_str(), s2, ann);
    println!("{}", result);
}

// 在一个函数里面结合泛型类型参数，trait bounds，和生命周期
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 生命周期省略 lifetimes elision
// fn first_world<'a>(s: &'a str) -> &'a str {}
// Rust 根据生命周期省略规则(lifetime elision rules)将显式的注解给干掉了
// 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes）
// 返回值的生命周期被称为 输出生命周期（output lifetimes）
// 生命周期省略规则(lifetime elision rules):
// 1.每一个引用的参数都有它自己的生命周期参数，如 fn foo<'a>(x: &'a i32), fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
// 2.如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数，如 fn foo<'a>(x: &'a i32) -> &'a i32
// 3.如果方法中有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut self，
// 那么 self 的生命周期被赋给所有输出生命周期参数（这使得方法更容易读写，因为只需更少的符号
// 如果编译器检查完这3条规则后仍然存在没有计算出生命周期的引用，编译器就会停止并生成错误
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// struct lifetimes annotation
// 这个注解声明了 ImportantExcept 的实例不能比 part 所引用的实例存在的更久
struct ImportantExcept<'a> {
    part: &'a str,
}

impl<'a> ImportantExcept<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Anntention please: {}", announcement);
        self.part
    }
}

// 函数的泛型生命周期
// 函数声明了一个生命周期 'a，入参和返参都需要在这个 lifetimes 里面有效，Rust 的借用检查器会检查有效性
// 即 lifetimes 'a 只取最短的那个交集
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// &i32
// &'a i32
// &'a mut i32

fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

//fn longest_cannot_compile<'a>(x: &str, y: &str) -> &'a str {
//    let result = String::from("really long string");
//    // returns a value referencing data owned by current function
//    // 返回函数内部 ownership 的值的引用会造成悬垂指针，
//    // 此时应该 move ownership，而不是一个 borrow reference
//    result.as_str()
//}
