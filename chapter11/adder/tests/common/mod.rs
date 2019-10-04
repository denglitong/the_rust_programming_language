// 为了不让 tests/common.rs 出现在测试输出中，我们创建 tests/common/mod.rs
// 这是一种 Rust 的命名规范，这样命名告诉 Rust 不要将 common 看作一个集成测试文件
// tests 目录中的子目录不会被作为单独的 crate 编译或作为一个测试结果部分出现在测试输出中

pub fn setup() {
    //
}
