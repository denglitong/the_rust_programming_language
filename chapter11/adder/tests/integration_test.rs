// 为了编写集成测试，你需要在项目根目录创建一个 tests 目录，和 src 同级，
// 可以在 tests 目录中创建任意多的测试文件，Cargo 会将每一个文件当做单独的 crate 来编译
// 并需要将 tests/ 下的任何代码标注为 #[cfg(test)]，这是因为 tests/ 是一个特殊的文件夹
// Cargo 只会在运行 cargo test 时编译这个目录中的文件

use adder::add_two;
use adder::Rectangle;

// 引入 tests/common 对应的 common 模块
mod common;

// 如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录
// 创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数
// 只有 crate 才会向其他 crate 暴露了可供调用和使用的函数，二进制 crate 只意在单独运行

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}
