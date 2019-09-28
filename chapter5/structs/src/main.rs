fn main() {
    // 声明了一个可变的 User struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("somenusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{:?}", user1);

    println!(
        "{:?}",
        build_user(
            String::from("denglitong@xiaomi.com"),
            String::from("denglitong")
        )
    );

    // 使用结构体更新语法从其他实例创建实例（未指定部分的字段和引用实例相同
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // 使用结构体更新语法：
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // 注意不能有,
    };
    println!("{:?}", user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 元组结构体（tuple structs）元组结构体有着结构体名称提供的含义，但没有具体的字段名只有字段的类型
// 当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的
// 这时像常规结构体那样为每个字段命名就显得多余和形式化了
// 定义元组结构体，以 struct 关键字和结构体名开头并后跟元组中的类型：
// 在其他方面，元组结构体实例类似于元组：可解构、可使用.来访问单独的值
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 没有任何字段的类单元结构体，他们被称为 类单元结构体（unit-like structs），因为他们类似于 () ，即 unit 类型
// 类单元结构体常常在你想要某个类型上实现 trait 但不需要在类型中存储数据的时候用到
struct Unit;

#[derive(Debug)]
// 给 User struct 启用 Debug 扩展，使得 User 可以使用 {:?} 打印出来
// Rust 不允许 struct 只将某个字段标记为可变
struct User {
    // 注意此处我们有意选择了 String 类型而不是 &str 字符串slice类型
    // 这是因为我们想要这个结构体能拥有它所有的数据，只要整个结构体是有效的其数据也是有效的
    // 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 声明周期（lifetimes）
    // 如果你尝试在结构体中存储一个引用而不指定声明周期将是无效的(如：username: &str这样)
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        //email, // field init shorthand，变量与字段同名时字段初始化简写语法
        //username,
        username,
        email, // 变量与字段同名和顺序无关
        active: true,
        sign_in_count: 1,
    }
}
