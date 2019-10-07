use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

// 1.自定义宏 #[derive] 只能用于结构体和枚举
#[derive(HelloMacro)]
struct AnotherPancakes;

// 2.类属性宏（Attribute）还可以用于更小的项，比如函数
// 比如可以创建一个名为 route 的属性用于注解 web 应用程序框架的函数
//#[route(GET, "/")]
//fn index() {}
// #[route] 属性将由框架本身定义为一个过程宏：
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
// 这里有两个 TokenStream 类型参数，第一个用于属性内容本身，也就是 Get, "/" 这部分
// 第二个属性是所标记的项，也就是 fn index() {} 这部分

// 3.类函数宏
// 类函数宏看起来就像函数调用的宏：
// let sql = sql!(SELECT * FROM posts WHREE id = 1);
// 这个宏的定义为：
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}

fn main() {
    Pancakes::hello_macro();
    AnotherPancakes::hello_macro();
}
