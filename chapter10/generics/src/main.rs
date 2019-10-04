// 函数将重复代码抽象到函数签名和函数实现中
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 为了参数化要定义的函数的签名中的类型，我们需要像给函数的值参数起名那样为这类型参数起一个名字
// 当需要在函数中使用一个参数时，必须在函数签名中声明这个参数以便编译器能知道函数体中这个名称的意义
// 为了定义泛型版本的 largest 函数，类型参数声明位于函数名称与参数列表中间的尖括号<>中
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        //  binary operation `>` cannot be applied to type `T`
        // 说明 > 这个操作不能满足所有可能的 T
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest2<T: PartialOrd + Clone>(list: &[T]) -> T {
    // 如果 T 为非 Copy 类型，则 clone 操作会潜在分配更多的内存从而可能导致程序变慢
    let mut largest = list[0].clone();

    for item in list.iter() {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}

fn largest3<T: PartialOrd>(list: &[T]) -> &T {
    // 返回一个引用
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 方法定义中的泛型
// impl<T> 含有 T，这样就可以在 Point<T> 上实现的方法中使用它了
// 在 impl 之后声明泛型，这样编译器就知道 Point<T> 中的 T 是泛型而不是具体类型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 为 Point<f32> 实例实现方法，而不是为泛型Point实例
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point3<T, U> {
    // 这里的 <V, W> 表示的是和 T,U 可能不相同的类型
    // impl<T,U> 声明的泛型是与结构体定义相对应的，而泛型参数 mix_up<V,W> 只是相对于方法本身的
    fn mix_up<V, W>(self, other: Point3<V, W>) -> Point3<T, W> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}
// Rust 的泛型相比使用具体类型并没有任何速度上的损失
// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程，
// 编译器寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum OptionI32 {
    Some(i32),
    None,
}
enum OptionF64 {
    Some(f64),
    None,
}

fn main() {
    //    let number_list = vec![34,50,25,100,65];
    //
    //    let mut largest = number_list[0];
    //
    //    for number in number_list {
    //        if number > largest {
    //            largest = number;
    //        }
    //    }
    //
    //    println!("The largest number is {}", largest);
    //
    //    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    //
    //    let mut largest = number_list[0];
    //
    //    for number in number_list {
    //        if number > largest {
    //            largest = number;
    //        }
    //    }
    //
    //    println!("The largest number is {}", largest);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    //    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    //    let result = largest_i32(&number_list);
    //    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("x {:?}, y {:?}", integer, float);
    //let wont_work = Point { x: 5, y: 4.0 }; // ^^^ expected integer, found floating-point number
    let integer_and_float = Point2 { x: 5, y: 4.6 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    let p3 = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);

    let largest_number = largest(&number_list);
    println!("The largest number is {}", largest_number);
    let largest_number = largest3(&number_list);
    println!("The largest number is {}", largest_number);
}
