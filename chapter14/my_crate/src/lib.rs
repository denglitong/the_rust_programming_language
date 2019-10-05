//! # My Crate
//!
//! `my_crate` 是一个使得特定计算更方便的
//! 工具集合

//! # Art
//!
//! 一个描述美术信息的库。

// 对于有很多嵌套模块的情况，使用 pub use 将类型重导出到顶级结果对于 crate 的使用者来说
// 将会是大为不同的体验；
// pub use 提供了解耦组织 crate 内部结构和与终端用户体现的灵活性

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// 采用 RGB 色彩模式的主要颜色。
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 采用 RGB 色彩模式的次要颜色。
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 等量的混合两个主要颜色，
    /// 来创建一个次要颜色。
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        //
        SecondaryColor::Green
    }
}

/// 将给定的数字加一
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(five));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
