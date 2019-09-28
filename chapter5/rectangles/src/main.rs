fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (width1, height1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_in_tuple(rect1)
    );

    let rect1 = &Rectangle {
        width: width1,
        height: height1,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_in_struct(rect1)
    );
    // {:?} 提示符告诉 println! 我们想要使用叫做 Debug 的输出格式
    // Debug 是一个 trait，它允许我们以一种对开发者有帮助的方式打印结构体
    // 不过我们必须为结构体显式选择这个功能，为此需要在struct的定义之前加上 #[drive(Debug)]
    println!("rectangle: {:?}", rect1); // 单行风格
    println!("rectangle: {:#?}", rect1); // 根据字段换行风格
    println!("rectangle width: {}, height: {}", rect1.width, rect1.height);

    // 方法（method） vs 函数（function）
    // 函数只是一段功能代码，没有绑定上下文，但方法是绑定到类型的功能实现
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // Rest 中的函数语法和 Go 里面的一样，
                     // 自动引用和解引用（automatic referencing and dereferencing）
                     // 它是这样工作的：当使用 object.something() 时，Rust 会自动的为 object 添加 &，&mut，或 *
                     // 以便使 object 与方法签名匹配，下面的调用是等效的：
                     // p1.distance(&p2);
                     // (&p1).distance(&p2);
                     // 在给出接受者和方法名的前提下，Rust 可以明确计算出方法是仅仅读取(&self)，
                     // 作出修改(&mut self)或是获取所有权(self)
                     // Rust 对方法接受者的隐式借用让所有权在实践中更友好
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("{}", sq.is_square());
}

// 原始版本
fn area(weight: u32, height: u32) -> u32 {
    weight * height
}

// 使用元组重构，但丢弃了长度、高度的信息，因为不确定哪个是长度高度
fn area_in_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
// 有点意思，有点像Java中的注解~
struct Rectangle {
    width: u32,
    height: u32,
}

// 为了使函数定义与 Rectangle 的上下文中，我们开始了一个 impl 快（implement）
// 一个结构体允许拥有多个 impl 快（虽然这是有效的语法，但是没有理由将方法分散到多 impl 快中）
impl Rectangle {
    // 绑定到类型的方法，也使用fn定义，但他们的第一个参数总是self，它代表调用该方法的类型实例
    // 依然要注意这里的 &self，表示只是不可变的借用，如果想要修改类型实例的数据，需要改成 &mut self
    // 通过仅仅使用 self 作为一个参数来使方法获取实例的所有权是很少见的，
    // 这种技术通常用在档方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    // 关联函数：impl允许在快中定义不以 self 作为参数的函数，这被称为 关联函数（associated functions），因为他们与结构体相关联
    // 他们仍是函数而不是方法，因为它们不作用于一个结构体的实例，比如 String::from() 就是一个关联函数
    // 关联函数经常被用作返回一个结构体新实例的构造函数，
    // 使用结构体名和::来调用这个关联函数，关联函数位于结构体的命名空间中，:: 语法用于关联函数和模块创建的命名空间
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // duplicate definitions for `area`
    //    fn area(&self) -> u32 {
    //        self.width * self.width
    //    }

    // duplicate definitions with name `area`
    //    fn area(&self, other: &Rectangle) -> u32 {
    //        self.width * other.height
    //    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// 我们希望借用结构体而不是获取它的所有权，所以这就是为什么函数签名和调用的地方有 &，表示引用
fn area_in_struct(other: &Rectangle) -> u32 {
    other.width * other.height
}
