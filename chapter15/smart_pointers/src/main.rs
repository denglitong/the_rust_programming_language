// 指针 pointer 是一个包含内存地址的变量的通用概念，这个地址引用指向一些其他数据
// Rust 中最常见的指针是第四章介绍的 引用（reference），使用 & 符号并解压了所指向的值

// 智能指针（smart pointers）是一类数据结构，它们的表现类似指针，但是也拥有额外的元数据和功能
// 智能指针起源于 C++，Rust 标准库中不同的智能指针提供了多于引用的额外功能，如 引用计数（reference counting） 智能指针

// 在 Rust 中，普通引用 和 智能指针 的一个额外区别是 引用是一类只借用数据的指针，
// 而 智能指针 在大部分情况下都 拥有 它们指向的数据
// 比如 String 或 Vec<T>，这些类型都属于智能指针因为它们拥有一些数据并允许你修改它们，
// 它们也带有元数据（比如它们的容量）和额外的功能或保证（如 String 的数据总是有效的 UTF-8 编码

// 智能指针通常使用结构体实现，区别于常规结构体的显著特征在于其实现了 Deref 和 Drop trait
// Deref trait 允许智能指针结构体表现的像引用一样
// Drop trait 允许我们自定义当智能指针离开作用域时运行的代码

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    // 最简单直接的智能指针是 Box<T>，它允许你将一个值放在堆上而不是栈上，留在栈上的则是指向堆数据的指针
    let b = Box::new(5);
    println!("b = {}", b);
    // 到这里 b 被释放，释放过程作用于 b 本身（位于栈上）和它所指向的数据（位于堆上）

    //    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    //
    //    let x = 5;
    //    let y = &x;
    //    assert_eq!(5, x);
    //    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // Box<T> 类型可以解引用，是因为底层实现了 Deref trait

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 底层调用 self.deref() 获得引用

    hello("Rust");
    let m = MyBox::new(String::from("Rust"));
    // MyBox::deref -> &String -> String::deref -> &str
    // 自动隐式解引用都发生在编译时，在运行时不会引入负担
    hello(&m);

    // 解引用强制多态与可变现交互
    // 类似于如何使用 Deref trait 重载不可变引用的 * 运算符，
    // Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符
    // Rust DerefMut 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：
    // 1. 当 T: Deref<Target=U>, &T => &U
    // 2. 当 T: DerefMut<Target=U>, &mut T => &mut U
    // 3. 当 T: Deref<Target=U>, &mut T => &U

    //    let c = CustomSmartPointer { data: String::from("my stuff")};
    //    let d = CustomSmartPointer { data: String::from("other stuff")};
    //    println!("CustomSmartPointer created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // 通过 Drop trait 和 Rust 的所有权系统，你无需担心之后清理代码，也无需担心意外的清理掉仍在使用的值
    // 因为这会造成编译器错误根本过不了编译，所有权系统确保引用总是有效的，也确保 drop 只会在值不再使用时被调用一次
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    //    // Rust 的多所有权 Rc<T> 引用计数智能指针
    //    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //    println!("count after creating a = {}", Rc::strong_count(&a));
    //    let b = Cons(3, Rc::clone(&a));
    //    println!("count after creating b = {}", Rc::strong_count(&a));
    //    {
    //        let c = Cons(4, Rc::clone(&a));
    //        println!("count after creating c = {}", Rc::strong_count(&a));
    //    }
    //    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // 自动解引用
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // 循环引用 导致内存泄露

    // Weak<T>
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent);
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
    // 这里 branch 离开作用域被释放，其中的引用计数会自动清理，leaf.parent 的 Weak<T> 并不会产生任何内存泄露！
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

// 增加从子节点到父节点的引用，需增加一个 parent 字段，问题是 parent 的类型不能包含 Rc<T>，
// 因为这样 leaf.parent 将会指向 branch 而 branch.children 包含 leaf 的指针，这会形成引用循环，
// 会造成 strong_count 永远也不会为0而导致内存泄露

// 父节点应该拥有子节点，如果父节点被抛弃那么子节点也应该被丢弃，
// 而子节点不应该拥有父节点，如果子节点丢弃，父节点应该依然存在，这正是弱引用的例子
// 所以 parent 使用 Weak<T> 类型而不是 Rc<T>，具体来说是 RefCell<Weak<Node>>

// Weak<T> 可以通过 Rc::downgrade<> 得到，表示 弱引用（weak reference），
// 不同于 Rc<T> 实例的 strong_count 加一，调用 Rc::downgrade 会将 weak_count 加一，
// 区别在于 weak_count 无需计数为0就能使 Rc 实例被清理
// (可以参考类比 Linux 文件系统中的 硬链接(inode号相同，指向同一数据块) vs 软链接（inode号不同，存储的数据库是执向另一个文件的inode号） ？)

// strong reference 代表如何共享 Rc<T> 实例的所有权，weak reference 并不代表所有权关系他们不会造成引用循环，
// 因为任何引入弱引用的循环一旦所涉及的强引用计数为0就会break
// 使用 Weak<T> 引用的值时需要确保其值仍然有效，可以调用 Weak<T> 的 upgrade 方法，这会返回 Option<Rc<T>>

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    // 我们希望 Node 能够拥有子节点，同时也希望通过变量来共享所有权以便可以直接访问树的每一个Node -> Vec<Rc<Node>>
    // 还希望能改变其他节点的子节点 -> RefCell<T> 内部可变性使得你能够通过不可变引用改变其内部的值
    children: RefCell<Vec<Rc<Node>>>,
}

// 内部可变性（interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时改变数据
// 这通常是借用规则所不允许的，而为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则
// 当可以确保代码在运行时会遵守借用规则，即使编译器不能保证的情况，也可以选择使用那些运用内部可变性模式的类型
// 所涉及的 unsafe 代码被封装进安全的API中，而外部类型仍然是不可变的
// RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候
// 类似的，Rc<T> RefCell<T> 只能用于单线程场景

// Box<T>, Rc<T>, RefCell<T>
// Rc<T> 允许相同数据有多个owner，Box<T> 和 RefCell<T> 有单一所有者
// Box<T> 允许在编译时执行不可变或可变借用检查，Rc<T> 进允许在编译时执行不可变借用检查，
// RefCell<T> 允许在运行是执行不可变或可变借用检查
// 因为 RefCellL<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改器内部的值

// 在不可变值内部改变值就是 内部可变性 模式（类似于 C++ 里面的 const array）
// 内部可变性：不可变值的可变借用

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // 当实例离开 scope 时 Rust 会自动调用 drop 方法
    // 显示的调用 drop 方法是不被允许的，Rust 会在实例离开作用域是自动调用 drop，
    // 如果允许显示调用那么会导致一个 double free 错误
    // 需提前销毁实例时需使用 std::mem::drop
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// 函数和方法的隐式解引用强制多态（deref coercions）
// 自动调用实现了 Deref 的类型的 deref 方法，把实参类型转换为形参所需的类型
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // 定义了用于此 trait 的关联类型
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
enum List {
    // 使用 Rcc 允许一个值有多个所有者，引用计数确保只要任何所有者依然存在则其值也保持有效
    // Rc<T> 允许通过不可变引用来在只读的程序的多个部分共享数据
    // 如果 Rc<T> 也允许多个可变引用，则会违反 任一时刻在相同位置不能出现多个可变借用 这一条规则，
    // 相同位置的多个可变借用可能造成数据竞争和不一致
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
