use crate::post::Post;
use oop::MyPost;

pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// 只有 对象安全（object safe）的 trait 才可以组成 trait 对象，需要满足：
// 1.返回值类型不为 Self
// 2.方法没有任何泛型类型参数
// 因为上述两个情况使得编译器不知道 trait 的实现是什么类型以及泛型参数的类型是什么
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // 此处如果使用泛型，则限制了 Screen 只能一次拥有一种类型的 components
    // 如果一次只需要同种类型的集合，则倾向于使用泛型和trait bould，因为其定义会在编译时才有具体类型进行单态化
    // 泛型使用 trait bould 时编译器进行单态化处理，单态化所产生的代码进行 静态分发（static dispatch）
    // 静态分发发生于编译器在编译时就知晓了调用了什么方法的时候，
    // 与之对应的是 动态分发（dynamic dispatch），这时编译器在编译时无法知晓调用了什么方法，动态分发编译器会生成
    // 运行时确定了调用了什么方法的代码
    // 当使用 trait 对象时，Rust 必须使用动态分发，因为编译器无法知晓所有可能用于 trait 对象代码的类型
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            // 只关心值所反映的信息而不是其具体类型，duck typing（叫起来像一只鸭子，那么它就是一直鸭子）
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button draw...")
    }
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox draw...")
    }
}

mod post;

fn main() {
    //    let screen = Screen {
    //        components: vec![
    //            Box::new(SelectBox {
    //                width: 75,
    //                height: 10,
    //                options: vec![
    //                    String::from("Yes"),
    //                    String::from("Maybe"),
    //                    String::from("No"),
    //                ],
    //            }),
    //            Box::new(Button {
    //                width: 50,
    //                height: 10,
    //                label: String::from("oK"),
    //            }),
    //            // Rust 会检测是否类型实现了 Draw trait
    //            // Box::new(String::from("Hi")),
    //        ],
    //    };
    //
    //    screen.run();

    //    let mut post = Post::new();
    //
    //    post.add_text("I ate a salad for lunch today");
    //    assert_eq!("", post.content());
    //
    //    post.request_review();
    //    assert_eq!("", post.content());
    //
    //    // post.reject();
    //
    //    post.approve();
    //    post.approve();
    //    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = MyPost::new();
    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

    println!("it works!");
}
