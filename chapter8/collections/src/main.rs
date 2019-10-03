// 不同于内建的数组和元组类型，Rust 中的 集合（collections）的数据是存储在堆上的
// 每种集合都有着不同能力和代价
// vector [Vec, VecDeque, LinkedList] (Sequences)
// string [String]
// hash map [HashMap, BTreeMap] (Maps)
// [HashSet, BTreeSet] (Sets)
// [BinaryHeap] (Misc)

use std::collections::hash_map::RandomState;
use std::collections::HashMap;

fn main() {
    // Vec 是泛型的，所以我们这里使用了一个类型注解
    let v: Vec<i32> = Vec::new();
    // 使用 vec! 宏创建的时候初始化，这样 Rust 就能基于初始化自动推断类型
    let v = vec![1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("vec: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    println!("vec: {:?}", v);
    let third = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(element) => println!("The third element is {}", element),
        None => println!("There is no third element"),
    }
    let third: Option<&i32> = v.get(2);
    println!("The third element is {:?}", third);

    // None 在前面才能自动类型推断，如果 if let third = None 会编译不过，因为 None 的类型推断不出来
    if let None = third {
        println!("There is no third element");
    } else {
        println!("The third element is {}", third.unwrap());
    }

    // let does_not_exist = &v[100]; // panic 'index out of bounds'
    let does_not_exist = v.get(100); // ok, return None if out of bounds

    // 再次复习不能在相同作用域中同时存在可变和不可变引用的规则
    let mut v = vec![1, 2, 3, 4, 5];
    // v immutable borrow occurs here
    let first = &v[0];
    // v mutable borrow occurs here
    // vector追加元素时在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配内存并将老的元素拷贝到新的空间中
    // 这时旧的不可变引用的内存就被释放了
    v.push(6);
    // immutable borrow later used here
    //    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    // 使用枚举来存储多种类型：再次复习 Rust 中的枚举是线性代数类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.2),
    ];
    println!("{:?}", row);

    // Rust 的核心语言中只有一种字符串类型：str, 字符串slice，它通常以被借用的形式出现 &str
    // 它们是一些存储在别处的 UTF-8 编码字符串数据的引用，比如字符串字面值被存储在程序的二进制输出中，字符串 slice也是如此；
    // String 类型是由标准库提供的，而没有写进核心语言部分
    // String 是可增长的、可变的、有ownership的、UTF-8编码的字符串类型
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    // .to_string 和 String::from 作用一样，如果选择取决于你的风格
    let s = String::from("initial contents");

    // 只要以UTF-8正确编码，String都能表示
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = "你好".to_string();

    // 更新字符串
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str 没有获取参数的所有权，所以在后面仍然可以使用它
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 ownership is moved, can not used later, s2 is just borrow, valid later
    // 此处的 + 运算符调用的函数签名看起来像：fn add(self, s: &str) -> String，
    // 注意此处的 self 获取了其所有权，这意味着使用了 add 后源字符串不再有效
    // 然而此处的 &s2 是 &String 类型，Rust 可以将 &String 强转成 &str，所以下面示例才能正常编译
    // 这是因为 Rust 使用了一个被称为 解引用强制多态（deref coercion） 的技术
    let s3 = s1 + &s2; // 该语句看起来好像生成了很多拷贝但实际上没有：这个实现比拷贝更高效
    println!("s3 is {}", s3);

    // format! 宏
    let s1 = String::from("tic");
    let s2 = "tac".to_string();
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    // 索引字符串
    let s1 = String::from("hello");
    //let h = s1[0]; // cannot be indexed by {integer}, is not implemented for String
    // String 是一个 Vec<u8> 的封装
    // 4个字节，每一个字母的UTF-8编码占用一个字节
    let len = String::from("Hola").len();
    println!("len is {}", len);

    let len = String::from("Здравствуйте").len();
    // len = 24，每个 Unicode 标量值需要两个字节存储，因此一个字符串字节值的索引并不总是对应一个有效的 Unicode 值
    println!("len is {}", len);
    //let answer = &hello[0];
    //println!("answer is {}", answer); cannot be indexed by {integer}

    // Rust 不允许使用索引获取 String 字符的原因：
    // 1. String 底层使用的是 Vec<u8> 存储的数据，而 String 采用的是 Unicode 编码，每个 Unicode 字面量需要2个字节，
    // 人们使用索引访问的时候，Rust 所能拿到的数据只是其中每个字节的数据，如 208，而不是对应的字面量，
    // 为了避免返回意想不到并造成不能立刻发现的 bug，Rust 拒绝编译这些代码
    // 2. 人们使用索引操作预期总是需要常数时间O(1)，但是对于 String 不可能保证这样的性能，
    // 因为 Rust 不得不检查从字符串的开头位置到索引的位置来确定这里有多少有效的字符

    // 字符串 slice
    let hello = "Здравствуйте";
    // 可以使用 [] 和一个 range 来创建包含特定字节的字符串 slice，但对应的 range 必须和 unicode 的双字节存储匹配
    // 不然会会 panic
    let s = &hello[0..4];
    // Зд
    println!("ok range, slice is {}", s);
    // invalid range, panic
    //let s = &hello[0..5];

    // 遍历字符串的方法
    // 如果需要操作单独的 Unicode 标量值，最好的选择是使用 chars 返回每个 char 类型的值
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    // न म स ् त े
    println!();
    // bytes 返回每一个原始字节
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    // 224 164 168 224 164 174 224 164 184 224 165 141 224 164 164 224 165 135
    println!();

    let mut scores = HashMap::new();
    // 类似于 Vector，HashMap必须存储同一种类型，即键的类型要相同，值的类型也要相同
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // 使用 zip 方法来创建一个元组的 vector，接着可以使用 collect 方法将这个元组 vector 转换成 HashMap
    let tuple_vec = teams.iter().zip(initial_scores.iter());
    // Zip { a: Iter(["Blue", "Yellow"]), b: Iter([10, 50]), index: 0, len: 2 }
    println!("tuple_vec is {:?}", tuple_vec);
    // 对于键和值的类型参数来说，可以使用下划线占位，Rust 能够根据 vector 中的数据类型自动推断
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    //  {"Blue": 10, "Yellow": 50}
    println!("scores is {:?}", scores);

    // hash map ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map is {:?}", map);
    // error: value borrowed here after move
    //println!("field_name {}, field_value {}", field_name, field_value);

    println!("Blue score is {:?}", scores.get(&String::from("Blue"))); // Some(10)，map返回的类型是 Option<V>

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 遍历 hash map，请注意这里的 &scores 是borrow，否则就变成 move 了
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // insert if absent
    scores.entry(String::from("Yellow")).or_insert(100);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert 返回键值的一个可变引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let numbers = vec![25, 1, 7, 5, 7, 6, 7, 4, 1, 3];
    println!("{:?}", numbers);
    println!("{:?}", number_exercise(&numbers));

    let s = String::from("first");
    println!("{}", string_to_pig_latin(&s));
    println!("{}", string_to_pig_latin(&String::from("")));
    println!("{}", string_to_pig_latin(&String::from("apple")));

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    company
        .entry(String::from("Engineering"))
        .or_insert(Vec::new())
        .push(String::from("Sally"));

    company
        .get_mut("Engineering")
        .unwrap()
        .push(String::from("Leon"));

    company
        .entry(String::from("Sales"))
        .or_insert(Vec::new())
        .push(String::from("Amir"));

    println!("{:?}", company);
    println!("{:?}", company.get("Engineering"));

    let mut all_employee = Vec::new();
    for val in company.values() {
        for v in val {
            all_employee.push(v.clone());
        }
    }
    all_employee.sort();
    println!("all employee: {:?}", all_employee);
}

// 字符串转换为 Pig Latin
fn string_to_pig_latin(s: &String) -> String {
    if s.len() == 0 {
        return s.clone();
    }

    let chars: Vec<char> = s.chars().collect();
    println!("chars {:?}", chars);

    let vowel_chars = vec!['a', 'e', 'i', 'o', 'u'];
    if vowel_chars.contains(&chars[0]) {
        return format!("{}-hay", s);
    }

    let s1: String = chars.iter().skip(1).collect();
    format!("{}-{}-ay", s1, chars[0])
}

// 平均数、中位数、众数
fn number_exercise(numbers: &Vec<u32>) -> (u32, u32, u32) {
    if numbers.len() == 0 {
        return (0, 0, 0);
    }

    let mut num = numbers.clone();
    num.sort();
    println!("num: {:?}", num);

    let mut median = 0;
    if num.len() % 2 == 0 {
        median = (num[(num.len() - 1) / 2] + num[num.len() / 2]) / 2;
    } else {
        median = num[num.len() / 2];
    }

    let mut sum = 0;
    let mut map = HashMap::new();
    for n in &num {
        sum += *n;
        *(map.entry(n).or_insert(0)) += 1;
    }

    let len: u32 = num.len() as u32;
    let mean = sum / len;

    let mut mode = 0;
    let mut max_count = 0;
    for (n, count) in map {
        if count <= max_count {
            continue;
        }
        max_count = count;
        mode = *n;
    }

    (mean, median, mode)
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
