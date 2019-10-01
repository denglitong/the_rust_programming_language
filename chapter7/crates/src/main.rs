// 模块 与 crate:
// crate 是一个二进制或库项目
// create根（crate root）是一个用来描述如何构建 crate 的文件
// 带有 Cargo.toml 文件的 包 用以描述如何构建一个或多个 crate，一个包中至多可以有一个库项目
// cargo new my-project 是在创建一个包，因为 Cargo 创建了 Cargo.toml，这意味着现在我们有了一个包
// Cargo 约定，如果在代表包的 Cargo.toml 的同级目录下包含 sr 目录且其中包含 main.rs 文件的话，
// Cargo 就知道这个包有一个与包同名的 二进制crate，且 src/main.rs 是 crate root
// 另一个约定是，如果包目录中包含 src/lib.rs，则包带有与其同名的 库crate，且 src/lib.rs 是 crate root
// crate root 文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目

// 一个包可以带有0个或一个 library crate（库crate） 和任意多个 binary crate（二进制crate），
// 一个包中必须带有至少一个 库crate 或者 二进制crate

// 如果包同时包含 src/main.rs 和 src/lib.rs 则它带有两个 crate：一个 库crate 和一个 二进制crate，并且同名
// 包可以带有多个二进制 crate，需将其文件置于 src/bin 目录，每个文件将是一个单独的 二进制crate

// https://users.rust-lang.org/t/what-is-the-difference-between-cargo-new-lib-and-cargo-new-bin/19009
// 类库项目 和 二进制项目 的区别，在于创建项目时约定生成的文件不同，比如 src/*.rs, Cargo.toml, .gitignore
// 另外，类库项目 可以在模块中被导入，比如可以调整到对应的代码位置，但是 二进制项目 因为只是一个可执行二进制文件，
// 所以无法像类库那样直接在代码中进行导入

// Rust 中的模块系统（the module system）:
// 模块：一个组织代码和控制路径私有性的方式
// 路径：一个命名项（item）的方式
// use：将路径引入作用域scope
// pub：使 item 变为共有
// as：将 item 引入 scope 时进行重命名（类似Golang里面的import xx xxx）
// glob: 将模块的所有内容引入作用域

// 使用分号而不是代码告诉 Rust 在另一个与模块同名的文件中加载模块的内容
// 文件中的定义内容不再包含 mod sound {} 而是直接模块内部的定义体
mod sound;

mod plant {
    pub struct Vegetable {
        // struct 中的字段默认是私有的，使用pub使其可以被访问
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        // 结构体的方法默认也是私有的，使用pub使其可以被访问
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    // 枚举默认也是私有的，pub 使其变为公有
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod performance_group {
    // 使用绝对路径的好处是代码可以移动位置而不改变引入路径，推荐使用指定绝对路径
    // 使用 pub use 将本模块引入的路径导出使其可见
    pub use crate::sound::instrument::woodwind;

    pub fn clarinet_trio() {
        woodwind::clarinet();
    }
}

// use 关键字引入路径，使其公有部分可见，省去冗长重复的调用路径
// self 指定相对路径
use self::sound::guitar;
// self 的相对路径开头不是必须的
//use sound::guitar;
// crate 指定决定路径
use crate::sound::instrument::woodwind;
// 虽然不推荐直接引入函数到scope，但对于struct, enum和其他项，习惯用法是指定到项的全路径：
use std::collections::HashMap;
// 如果 use 语句会将两个同名的项引入scope，这是不允许的，必须使用父模块，然后通过模块名::项 的方式使用
//use std::fmt;
//use std::io;

use std::fmt::Result;
//use std::io::Result; // Result re-imported here
// as 关键字重命名引入scope的type
use std::io::Result as IoResult;

//fn func1() -> fmt::Result {}
//fn func2() -> io::Result<()> {}
//fn func3() -> IoResult<()> {}

// 使用外部包
use rand::Rng;
// use 嵌套路径来消除大量的 use 行
use std::{cmp::Ordering, io};
//use std::io;
//use std::io::Write;
// 上面两行等价于：
//use std::io::{self, Write};

// 通过 * glob 运算符将所有的共有定义引入作用域，需要注意使用 * 导入所有公有包难以推导域中有什么名称以及在何处定义的
use std::collections::*;

fn main() {
    // 相对路径
    sound::instrument::woodwind::clarinet();
    // 绝对路径(模块有一个叫做 crate 的 root)
    crate::sound::instrument::woodwind::clarinet();

    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    let order1 = menu::Appetizer::Soup;
    println!("{:?}", order1);

    // use
    // 习惯用法是引用到函数的模块路径，然后通过模块::函数名调用
    woodwind::clarinet();
    // 不推荐直接将函数引入scope
    guitar();

    performance_group::clarinet_trio();
    // pub use
    performance_group::woodwind::clarinet();

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret number: {}", secret_number);
}
