// 定义了一个名为 sound 的模块，其包含了名为 guitar 的函数
// 模块可用嵌套
// 路径 path 是名称name的同义词，不过它用于文件系统语境，另外 函数，结构体和其他项可能会有多个路径指向相同的路径 path

// 模块是 Rust 中的 私有性边界（privacy boundary）
// 所有项默认是私有的（函数、方法、结构体、枚举、模块和常量）
// pub 关键字声明为共有
// 不允许使用定义于当前模块的子模块中的私有代码
// 允许使用任何定义于父模块或当前模块中的代码

// src/source/instrument.rs
// 在 Rust 中模块是一个树结构，再一次和文件目录的结构相同
pub mod instrument;

// public
pub fn guitar() {
    println!("guitar");
}
