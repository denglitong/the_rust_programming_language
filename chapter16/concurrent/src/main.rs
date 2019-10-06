// 并发编程（concurrent programming），代表程序的不同部分相互独立的执行
// 并行编程（parallel programming），代表程序不同部分同时执行
// Rust 的所有权和类型检查可以让很多并发编程的运行时错误在编译时发现，可以做到 fearless concurrency

// 线程模型：

// 进程（process）是操作系统中调度管理的最小单元，进程内部可以拥有多个同时运行的独立部分，被称为 线程（thread）
// 线程的好处：可以改善性能，因为可以同时执行多个任务
// 线程的坏处：
// 1.会增加复杂性：
// 竞争状态（race conditions）
// 死锁（deadlocks）
// 只会发生在特定情况且难以稳定重现和修复的bug

// 1:1 模型
// 一个OS线程对应一个语言线程，很多操作系统都提供了创建新线程的API
// M:N 模型
// M个语言线程对应N个OS线程（比如 go 里面的 go routine 协程）

// 每一个模型都有其优势和取舍，对于 Rust 来说最重要的取舍就是运行时（Runtime）支持
// M:N模型需要更大的语言运行时来管理这些线程，为此 Rust 标准库只提供了 1:1 线程模式实现
// Rust 需要做到几乎没有运行时，同时为了保持高性能必须能够调用 C 语言

// 消息传递（message passing），这个思想来源于 Go：不要共享内存来通讯，而是要通讯来共享内存；
// 通道（channel），发送者（transmitter），接受者（receiver）
// 当发送者或接受者任一个被丢弃时可以认为通道被 关闭（closed）了

use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    //    let handle = thread::spawn(|| {
    //        for i in 1..10 {
    //            println!("hi number {} from the spawned thread!", i);
    //            thread::sleep(Duration::from_millis(1))
    //        }
    //    });
    //
    //    // handle.join();
    //
    //    for i in 1..5 {
    //        println!("hi number {} from the main thread!", i);
    //        thread::sleep(Duration::from_millis(1));
    //    }
    //
    //    // 等待线程结束，blocking
    //    // handle.join();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // value used here after move
    // drop(v);

    handle.join();

    // 多个生产者，单个消费者 multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("tx1: more"),
            String::from("tx1: messages"),
            String::from("tx1: for"),
            String::from("tx1: you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // println!("val is {}", val); // value borrow after move
        let vals = vec![
            String::from("tx: hi"),
            String::from("tx: from"),
            String::from("tx: the"),
            String::from("tx: thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    // recv 方法会阻塞主线程直到从 channel 中接收一个值
    // try_recv 不会阻塞主线程，它立刻返回一个 Result<T, E>：Ok包含可用的信息，Err代表此时没有任何消息
    // 一般 try_recv 和轮询一起使用直到检查到有消息到来
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }

    // 互斥器（mutex, mutual exclusion）, lock, guarding
    let m = Mutex::new(5);
    {
        // lock 方法会阻塞当前线程，直到我们拥有锁为止
        // 如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败，这时 unwrap 会使线程 panic
        // lock 返回一个 MutexGuard 的智能指针，实现了 Deref 自动解引用 和 Drop 离开作用域时自动释放锁
        let handle = m.lock();
        let mut num = handle.unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // 多线程下的互斥锁与所有权
    //    let counter = Mutex::new(0);
    //    let mut handlers = vec![];
    //
    //    let handle = thread::spawn(move || {
    //        let mut num = counter.lock().unwrap();
    //        *num += 1;
    //    });
    //    handlers.push(handle);
    //
    //    let handle = thread::spawn(move || {
    //        // value used here after move
    //        let mut num2 = counter.lock().unwrap();
    //        *num2 += 1;
    //    });
    //    handlers.push(handle);
    //
    //    println!("Result: {}", *counter.lock().unwrap());

    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        // Rc<T> 并不能安全的在线程间共享，the trait bound `Send` is not satisfied
        // 因为 Rc<T> 底层并没有任何并发原语来保证改变计数的操作不会被其他进程打断
        // let counter = Rc::clone(&counter);
        // 而 Arc<T> 正是一个类似 Rc<T> 但可以安全的用于并发环境的类型，
        // A 代表了 atomic，这是一个原子引用计数类型（atomically reference counted）
        // Arc<T> 和 Rc<T> 有着相同的API
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(h);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // 使用 Sync 和 Send trait 的可扩展并发
    // Rust 语言核心本身对并发知道的很少，并发功能大多是标准库提供的
    // 但是 Rust 语言核心有2个并发概念是内嵌的：std::marker 中的 Sync 和 Send trait

    // 实现了 Send trait 的类型表明其所有权可以在线程间传递，
    // 几乎所有的 Rust 类型都是 Send 的，不过有一些例外如 Rc<T>，这是不能 Send的，
    // 因为如果克隆了 Rc<T> 的值并尝试将克隆的所有权移到另一个线程，这2个线程都可能同时更新引用计数，
    // 为此，Rc<T> 被实现为用于单线程场景，这时不需要为拥有线程安全的引用计数而付出性能代价
    // 因此，Rust 类型系统和 trait bound 确保永远也不会意外的将不安全的 Rc<T> 在线程间发送
    // 任何完全由 Send 的类型组成的类型也会自动被标记为 Send，几乎所有基本类型都是 Send（除了后面要讨论的野指针）

    // 实现了 Sync trait 的类型表明其可以安全的在多个线程中拥有其值的引用，
    // 对于任意类型 T，如果 &T 是 Send 的话，那么 T 就是 Sync 的，这意味着其引用就可以安全的发送到另一个线程
    // 类似于 Send, 基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的

    // 智能指针 Rc<T> 不是 Sync 的，和它不是 Send 同一个原因，
    // RefCell<T> 和 Cell<T> 系列类型也不是 Sync 的, RefCell<T> 在运行时所进行的借用检查也不是线程安全的
    // Muxtex<T> 是 Sync 的，它可以被用来在多线程中共享访问
}
