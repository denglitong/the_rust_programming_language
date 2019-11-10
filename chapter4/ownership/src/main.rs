// 栈（Stack）与堆（Heap）
// 在 Rust 这样的系统编程语言中，值位于栈上海市堆上在更大程度上影响力语言的行为以及为何必须做出这样的抉择
// 栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同，栈以放入值的顺序存储值并以相反顺序取出值，
// 这叫做 后进先出（last in, first out），想象跌盘子，在顶部增加，从顶部拿走
// 增加数据叫做 进栈（pushing onto the stack），移除数据叫做 出栈（popping off the stack）
// 栈的操作是十分快速的，这主要得益于它存取数据的方式：因为数据存取的位置总是在栈顶而不需要寻找一个位置存放或读取
// 另一个让操作栈快速的属性是，栈中的所有数据都必须占用已知且固定的大小
// 在编译是大小未知或大小可能变化的数据，要改为存储在堆上
// 堆是缺乏租住的：当向堆放入数据时，你要请求一定大小的空间，操作系统在堆的某处找到一块足够大的空位，然后标记为已使用
// 并返回一个表示该位置地址的 指针（pointer），这个过程叫做 在堆上分配内存（allocating on the heap）,
// 有时简称为 分配（allocating）
// 将数据推入栈中并不认为是分配，因为指针的大小是已知并且是固定的，
// 你可以将指针存储在栈上，不过当实际需要数据时，必须访问指针
// 跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间
// 这些问题正是所有权（ownership）系统要处理的

/// 所有权的规则：
/// 1.Rust 中的每一个值都有一个被称为其 所有者（owner）的变量 Each value in Rust has a variable that's called its owner
/// 2.值有且只有一个所有者 There can only be one owner at a time
/// 3.当所有者（变量）离开作用域，这个值被丢弃 When the owner goes out of scope, the value will be dropped

mod strings;

fn main() {
    // variable scope
    {
        // 变量 s 绑定到一个字符串字面值，这个值是硬编码到程序代码中的，
        // 这个变量从声明的点开始到当前 作用域 结束时都说有效的
        let s = "hello";
    }
    // {} 就是变量 s 的作用域

    // 复习：
    // 简单变量类型： i32, u32, usize, f32, char字符, &str字符串, bool布尔, tuple元组, array数组
    // 简单变量类型存储在栈上，当离开作用域时值被移出栈

    // 使用 String::from 从字面值创建String类型
    // 字符串字面值在编译期就知道其内容（不可变），所以文本被直接硬编码进最终的可执行文件中，这使得字符串字面值快速高效
    // 对于 String 类型，需要支持可变、可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放，这意味着：
    // 1.必须在运行时向操作系统请求内存
    // 2.需要一个当我们处理完 String 时将内存返回给操作系统的方法
    // Rust: 内存在拥有它的变量离开作用域后就被自动释放，Rust 为我们自动调用了一个特殊的函数 drop
    {
        let mut s = String::from("hello"); // 从此处起 s 是有效的
        s.push_str(", world!"); // 使用 s
        println!("{}", s); // 使用 s
    } // 此作用域已结束，s 不再有效
      // 在 C++ 中，这种 item 在生命周期结束是释放资源的模式被称作 资源获取即初始化（Resource Acquisition Is Initialization, RAII）
      // 这个模式对编写 Rust 代码的方式有着深远的影响，现在它看起来很简单，
      // 不过在复杂的场景下代码的行为可能是不可预测的，比如当多个变量使用在堆上分配的内存时。。。

    // 变量和数据交互的方式1：移动（move）
    let x = 5; // 将 5 绑定到 x
    let y = x; // 生成一个值 x 的拷贝并绑定到 y，因为整数是已知固定大小的简单值，所以这两个5被放入了栈中
    println!("{}", x);
    let s1 = String::from("hello");
    let s2 = s1; // 我们从栈上拷贝了s1的ptr、len、capacity，但并没有复制ptr所指向的堆上数据
                 // 这看上去像其他语言的浅拷贝，但是 Rust 会将第一个变量置为无效，所以这个操作被称为 移动（move）
                 // 这里隐含了一个设计选择：Rust 永远不会自动创建数据的"深拷贝"
                 // println!("{}", s1); // s1 is moved to s2, s1 is out of scope here
                 // 当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存，这是一个叫做 二次释放（double free）的错误
                 // 两次释放相同的内存会导致内存污染，他可能导致潜在的安全漏洞
                 // String
                 // name    | value      index | value
                 //  ptr         ------>   0       h
                 //  len         5         1       e
                 // capacity     5         2       l
                 //                        3       l
                 //                        4       l

    // 变量和数据交互的方式2：克隆（clone）
    // 如果我们确实需要深度复制 String 中堆上的数据而不仅仅是栈上的数据，可以使用一个 clone 的通用函数
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 当出现 clone 调用时，你知道一些特定的代码被执行并且这些代码可能相当消耗资源
    println!("s1 = {}, s2 = {}", s1, s2);

    // 这里没有调用clone，原因是像整型这样的在编译是已知大小的类型被整个存储在栈上，所以值的拷贝是快速的
    // 这意味着没有理由在创建变量y后使x无效，所以这里没有深浅拷贝的区别
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Rust 有一个叫做 Copy trait的特殊注解，可以用在类似整型这样的存储在栈上的类型上
    // 如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用
    // 如下是一些 Copy 的类型：
    // 所有整数类型，如 u32
    // 布尔类型 bool
    // 所有浮点数类型，如 f64
    // 字符类型，char
    // 元组 tuple 当且仅当其包含的类型也都是 Copy 的时候，如(i32, i32)，但(i32, String)就不是

    // 所有权与函数
    // 将值传递给函数在语义上与给变量赋值相似，向函数传值可能会移动或复制，就像赋值语句一样
    let s = String::from("hello");
    takes_ownership(s); // s的值移动到函数里 ...
                        // 所以到这一行不再有效
                        // print!("{}", s); // error: value borrowed here after move

    let x = 5; // x 进入作用域
    makes_copy(x); // 但 i32 是 Copy 的，所以在后面可以继续使用 x

    // 返回值与作用域 返回值也可以转移所有权
    let s1 = gives_ownership(); // gives_ownership 将返回值移給 s1
    let s2 = String::from("hello"); // s2 进入作用域

    /*
    变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
    当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动到另一个变量所有
    在每一个函数中多获取所有权并接着返回所有权有些啰嗦...
    如果我们想要函数使用一个值但不获取所有权该怎么办呢？每次传进去再返回来就有点烦了。。
    我们可以使用元组返回多个值，但显得形式主义，对此，Rust 提供了一个功能，叫做 引用（references）
    */
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    // 使用 & 传递引用，它们允许你使用值但不获取其所有权
    // &s1 语法让我们创建一个 指向 值 s1 的应用，但是并不拥有它
    // 因为不拥有这个值，当引用离开作用域是其指向的值也不会被丢弃,仅仅丢掉该引用本身
    // （如果引用的值比引用被提前 drop 编译器如何捕捉到？）
    let len = calculate_length2(&s1);
    println!("The length of '{}' is {}.", s1, len);
    /*
        s                   s1
    name | value     name    | value     index | value
     ptr     -------->ptr        -------->  0       h
                     len        5           1       e
                     capacity   5           2       l
                                            3       l
                                            4       o
    */

    // 可变引用：必须是 mut 类型，必须创建一个 &mut 的可变引用
    let mut s = String::from("hello");
    change2(&mut s);
    println!("{}", s);
    // 可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用
    // 这个限制的好处是 Rust 可以在编译时就避免数据竞争：比如
    //  两个或多个指针访问同一数据
    //  至少有一个指针被用来写入数据
    //  没有同步数据访问的机制
    // 数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复
    // Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！
    //let r1 = &mut s;
    //let r2 = &mut s; // error: second mutable borrow occurs here
    //println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以此后又可以新建可变引用了
    let r2 = &mut s;
    // 类似的规则也存在与同时使用可变与不可变引用中：即不能在特定作用域特定数据同时存在可变引用与不可变引用
    //let r1 = &s; // no problem
    //let r2 = &s; // no problem
    //let r3 = &mut s; // big problem! cannot borrow `s` mutable because it is also borrowed as immutable
    //println!("{}, {}, {}", r1, r2, r3);
    let r1 = &s; // 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
    let r2 = &s;
    println!("{} and {}", r1, r2); // 函数入参，移动
    // 此位置之后 r1 和 r2 不再使用（则r1 r2作用域到此为止）
    let r3 = &mut s; // 没问题
    change2(r3);
    println!("{}", r3);
    // println!("{}", r1); // 若在此处用到r1，则r1的作用域到这一行，就会包含了&mut的作用域，从而报错
    // cannot borrow `s` as mutable because it is also borrowed as immutable

    // 悬垂引用（Dangling References）释放了内存后依然有位置保留了指向它的指针
    // Rust 编译器确保引用永远也不会编程悬垂状态：
    // 当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域
    //let reference_to_nothing = dangle(); // missing lifetime specifier

    /*
    复习：
    对特定作用域特定数据，要么只能有一个可变引用，要么只能有多个不可变引用
    引用必须总是有效（不会有悬垂指针）
    */

    // 另一个不拥有所有权的数据类型是 slice
    // slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合（很像golang里面的slice）
    let s = String::from("hello");
    let hello = &s[0..5]; // .. 表示[)，如果需要表示闭区间，需要使用 ..=
    //let world = &s[6..=11]; // thread main panicked at 'byte index 6 is out of bounds of `hello`
    let slice = &s[0..=2];
    let slice = &s[..=2];
    let slice = &s[3..s.len()];
    let slice = &s[..s.len()];
    let slice = &s[..];
    // 字符串 slice range的索引必须位于有效的UTF-8字符边界内，
    // 如果尝试从一个多字节字符的中间位置创建字符串slice，则程序将会因错误而退出
    // 字符串 slice 的类型声明协作 &str
    let s = String::from("hello world");
    let world = first_word(&s);
    // s.clear(); // error! 当拥有某值的不可变引用时，就不能再获取一个可变引用，此处 clear 需要清空 String，它尝试获取一个可变引用，它失败了
    // Rust 不仅使得我们的API简单易用，也在编译时就消除了一整类的错误！
    // println!("the first world is: {}", world);

    // 字符串字面值就是 slice，字符串字面值存在二进制文件中，它是一个指向二进制程序中特定位置的 slice，
    // 这也是为什么字面值是不可变的，&str 是一个不可变引用
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    let s = "Hello world!";
    let word = first_word(&s[..]);
    let word = first_word(s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice); // [2, 3]

    // 所有权系统影响了 Rust 中很多其他部分的工作方式
}

// 返回单词结尾的索引
//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes(); // 转化为字节数组
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//    s.len()
//}

// fn first_word(s: &str) -> &str {}
// 定义一个获取字符串 slice 而不是 String引用 的函数使得我们的API更具统一并且不会丢失任何内容
//fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//} // 在这里 s 离开作用域并被 drop，此时 s 的引用的生命周期不能长于 s，编译不过
// help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 返回 s，所有权被移交，所以没有值被释放
}

// 正如变量默认是不可变的，引用也一样，默认不允许修改引用的值
//fn change(some_string: &String) {
//    some_string.push_str(", world"); // cannot borrow immutable variable as mutable
//}
fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: String) -> (String, usize) { // 获取所有权，再返回所有权
    let length = s.len();
    (s, length)
}

// 我们将获取引用作为函数参数称为 借用（borrowing）
fn calculate_length2(s: &String) -> usize { // 以一个对象的引用作为参数而不是获取值的所有权，s 是对 String 的引用
    s.len()
} // 这里 s 离开了作用域，但因为它并不拥有其引用值的所有权，所以其引用值不会发生 drop

fn gives_ownership() -> String { // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给被调用的函数
}

// takes_and_give_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里 some_string 移出作用域并调用 drop 方法，占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer)
} // 这里 some_integer 移出作用域，不会有特殊操作（可理解为函数出栈起参数自动被弹出？）

