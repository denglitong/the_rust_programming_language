fn main() {
    // 1.变量和可变性
    // 变量默认是不可改变的（immutable），这是利用 Rust 提供的安全性和简单并发性来编写代码的众多方式之一
    // 当变量不可变时，一旦值被绑定到一个名词上，你就不能改变这个值
    // Rust 编译器保证，如果生命一个值不会变，它就真的不会变
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; // cannot assign twice to immutable variable
    //println!("The value of x is: {}", x);

    // 可变性也是非常有用的，使用 mut 将变量声明为可变的
    let mut y = 5;
    y = y + 1;
    println!("The value of y is: {}", y);

    // 我们可以定义一个与之前变量同名的新变量，新变量会 隐藏（shadow） 之前的变量
    // 这意味着使用这个变量时会看到第二个值，可以多次隐藏
    // shadow 与 mut 是有区别的，如果没有使用 let 关键字，在重新赋值时会导致编译错误
    // 使用 let 我们可以进行一些计算，不过计算完后的变量仍然是不可变的
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // mut 与 shadow 的另一个区别是，当再次 let 时，实际上创建了一个新变量
    // 我们可以改变值的类型，但复用同一个名字，shadow 使我们可以不必使用不同的名字
    let spaces = "Rust";
    println!("The value of spaces is: {}", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // 如果尝试使用 mut，将会得到一个编译时错误，因为 Rust 有强静态类型检查
    // let mut spaces = "Rust";
    // spaces = spaces.len(); // 不能改变对象的类型

    // 常量是绑定到一个名词的不允许改变的值，它总是不可变的
    // 声明常量使用 const 而不是 let，并且必须注明值的类型
    // 常量只能被常量表达式初始化，而不能是函数调用的结果或运行时计算的值
    // 在声明它的作用域之中，常量在整个声明周期中都有效
    // 将遍布于应用程序中的硬编码值（魔法常量）声明为常量，能帮助表达程序的意图
    const MAX_POINTS: u32 = 100_000;
    println!("The constant value of MAX_POINTS is: {}", MAX_POINTS);

    // 2.数据类型
    // Rust 中每一个值都属于某一个 数据类型（data type）
    // Rust 中有两类数据类型子集：标量（scalar）和复合（compound）（向量坐标轴？(值，Err)）
    // Rust 是静态类型（statically typed）语言，在编译期就必须知道所有变量的类型
    // 需要显示添加类型推定 : u32，因为编译器需要我们提供更多信息
    let guess: u32 = "42".parse().expect("Not a number");
    println!("The value of guess is: {}", guess);

    // 标量类型（scalar），代表一个单独的值，Rust 有四种基本的标量类型：
    // 整型、浮点型、布尔类型、和字符串类型
    // 8-bit   i8  u8
    // 16-bit  i16 u16
    // Rust 的数字类型默认是 i32，它通常是最快的，升职在64位系统上也是
    // 32-bit  i32 u32
    // 64-bit  i64 u64
    // isize 和 usize 类型依赖运行程序的计算机架构，
    // 64位架构上它们是64位的，32位架构上它们是32位的，主要作为某些集合的索引
    // arch    isize usize
    let pos: u8 = 0;
    println!("The value of pos is: {}", pos);

    // 浮点数（floating-point numbers）
    // f32 单精度
    // f64 双精度 默认类型是 f64，在现代 CPU 中与 f32 速度几乎一样不过精度更高
    let x = 2.0; // f64
    let y: f32 = 3.0;
    println!("The value of x is: {}, y is: {}", x, y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("The sum is: {}", sum);
    println!("The difference is: {}", difference);
    println!("The product is: {}", product);
    println!("The quotient is: {}", quotient);
    println!("The remainder is: {}", remainder);

    // bool
    let t = true;
    let f: bool = false;
    println!("The t is: {}", t);
    println!("The f is: {}", f);

    // Rust 的 char 代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容
    // 在 Rust 中，拼音字母（Accented letters）、中文、日文、韩文等字符、emoji（绘文字）以及零长度的空白字符都是有效的 char 值
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("c is: {}", c);
    println!("z is: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    // 复合类型（Compound types）可以将多个值组合成一个类型
    // Rust 有两个原生的复合类型：元组（tuple）和 数组（array）
    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式，
    // 我们使用包含在圆括号内的逗号分隔的值列表来创建一个元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 元组是一个单独的复合元素，为了才能够元组中获取单个值，
    // 可以使用模式匹配（pattern matching）来解构（destructure）元组值:
    let (x, y, z) = tup;
    println!("The value os x, y, z is: {}, {}, {}", x, y, z);
    // 除了视频模式匹配解构外，也可以使用点号（.）后跟值的索引 tuple indexing 来直接访问：
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{} {} {}", five_hundred, six_point_four, one);

    // 数组每个元素的类型必须相同，声明后长度不能改变
    // 数组是一整块分配在栈上的内存，使用 array indexing 来访问
    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    // 如果数组越界则编译失败（runtime panic）
    println!("first : {}, second: {}", first, a[1]);
}
