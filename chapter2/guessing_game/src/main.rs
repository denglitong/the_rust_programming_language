use std::io;
// Rng 是一个 trait，它定义了随机数生成器应实现的方法
use rand::Rng;
// 同 Result 一样，Ordering 也是一个枚举，它的成员是 Less, Greater, Equal
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // rand::thread_rng 函数提供实际使用的随机数生成器
    // 它位于当前执行线程的本地环境中，并从操作系统取 seed
    // gen_range 产生一个[)区间的随机数
    // 如何知道依赖的 trait 的API文档？在项目目录构建依赖包文档：cargo doc --open
    // 自动类型推断，u32
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        // let 用来创建并绑定变量
        // 在 Rust 中，变量默认是不可变的，在变量名前使用 mut 来使一个变量可变：
        // String::new() 是标准款提供的字符串类型，是 UTF-8 编码的可增长文本快
        // ::new 表示 new 是 String 类型的一个 关联函数（associated function）
        // 关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个特定实例
        // 关联函数相当于 Java/C++ 中的静态方法（static method），
        // 而针对特定实例的方法在 Ruby 中出现过
        // 另外此处的 类型String，相比之 类String（Class String），
        // 和 Mysql 中的表（table）切换到 ES 的 index 转换有点类似
        // Rust 有一个静态强类型系统，同时也有类型推断
        let mut guess = String::new();

        // read_line 的工作是将用户的输入存入一个字符串，所以此处的 guess 需要是可变的
        // & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，即指针
        // 引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用
        io::stdin()
            // read_line 返回一个值 io::Result，在 Rust 中叫做 Result 类型
            // Result 类型是 枚举（enumerations）,即 enum，这里的枚举成员值有 Ok, Err
            // Ok 表示操作成功，内部包含成功时产生的值
            // Err 意味着操作失败，并且包含失败的前因后果
            .read_line(&mut guess)
            // expect 是 Result 类型实例的方法
            // 如果 Result 的值是 Err，expect 会导致程序奔溃
            // 如果 Result 的值是 Ok，expect 会获取 Ok 的值并原样返回
            .expect("Failed to read line");

        // 这里创建了一个叫做 guess 的变量，不过 Rust 允许用一个新值来 隐藏（shadow）之前的值
        // 这个功能常用在需要转换值类型的场景，它允许我们复用 guess 变量的名字而不是被迫创建2个不同的变量
        //let guess: u32 = guess
        // trim() 方法取出首尾的空白字符，用户输入5并按下enter键时，guess看起来像 5\n，trim()会移除\n
        //.trim()
        // 字符串的 parse 方法将字符串解析成数字，因为可以解析多种数据类型所以需要显示指定数字类型，此处是 let guess: u32
        //.parse()
        //.expect("Please type a number!");

        // 忽略非数字输入并继续
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ 是一个通配符，本例用来匹配所有 Err 值，continue 是进行下一次 loop
            Err(_) => continue,
        };

        // println!("Your guessed: {}", guess);

        // cmp 方法用来比较两个值并可以在任何可比较的值上调用，它获取一个被比较值的引用
        // 一个 match 表达式由 分支（arms）构成
        // 一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应执行的代码
        // match 结果和模式是 Rust 中强大的功能，它体现了代码可能遇到的多种清晰，
        // 并帮助你确保没有遗漏处理
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // 分支1，分支如果没有匹配就继续向下走
            Ordering::Greater => println!("Too big!"), // 分支2，分支匹配后执行完代码就会自动退出match
            Ordering::Equal => {
                println!("You win!");
                break; // 退出 loop 循环
            }
        }
    }
}
