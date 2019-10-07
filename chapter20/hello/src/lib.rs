use crate::Message::NewJob;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

// 为 FnOnce() 实现 FnBox trait
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        // 将闭包移出 Box<T> 并调用此闭包
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // 创建一个线程的时候就需要给定对应的闭包，这里用空闭包填充
        let thread = thread::spawn(move || loop {
            // 在 receiver 上调用 lock 来获取互斥器
            // 调用 recv 会阻塞当前线程，所以如果发送端里还没有任务，线程就会一直阻塞到有可用的任务来临
            // Mutex<T> 确保一次只有一个 Worker 线程尝试请求任务
            // 有个微妙的原因：Mutex结构体中没有pub unlock方法，
            // 因为锁的ownership依赖 lock 方法返回的 LockResult<MutexGuard<T>>中MutexGuard<T>的生命周期
            // 这允许借用检查器在编译时确保不会在没有持有锁的情况下访问Mutex守护的资源
            // 这里lock方法返回的MutexGuard在let job语句结束之后就立刻被丢弃了而不是一直持有锁的ownership，
            // 这就允许并发处理多个请求了；
            // 而如果将 let job 写到 while 循环的判断语句中，那么因为 scope 的原因使得闭包执行完之前其 lifetimes 都是有效的
            // 从而没有释放锁导致串行执行
            let message = receiver.lock().unwrap().recv().expect("获取消息锁失败");
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job, executing.", id);
                    // cannot move a value of type dyn std::ops::FnOnce() + std::marker::Send: the size of
                    // dyn std::ops::FnOnce() + std::marker::Send cannot be statically determined
                    // 此处为了调用存储在 Box<T> 中的 T，即 FnOnce 闭包，该闭包需要能将自己移出 Box<T>，
                    // 因为当调用这个闭包时，它获取 self 的所以权。通常来说将值移出 Box<T> 是不被允许的因为 Rust 不知道
                    // T 的值有多大该给他分配多少内存
                    // (*job)();

                    // 这里给 FnOnce() 实现了一个 trait，在这个 trait 里面我们使用 self: Box<Self> 来获取闭包的所有权，
                    // 一旦获取闭包的所有权我们就可以调用它了
                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    /// 创建线程池
    ///
    /// 线程池中线程的数量
    ///
    /// # Panics
    ///
    /// `new` 函数在 size = 0 时会 panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // FnOnce 仍然需要后面的 ()，因为这里的 FnOnce 代表一个没有参数也没有返回值的闭包
    /// 发送闭包消息让线程执行
    ///
    /// 闭包
    ///
    /// # Panics
    ///
    /// `execute` 函数在发送闭包消息失败时会 panic
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .send(Message::NewJob(job))
            .expect("发送消息失败");
    }
}

// 优雅停机：对线程池实现 Drop trait 并 join 各个线程等待其结束
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers down worker");

        // 这里之所以使用2个for循环将 Terminate 消息和等待worker执行完分开，
        // 是为了防止 Terminate 消息被其他线程接受后再调用本线程的 join 时会因为得不到消息锁而一直等待，
        // 因为此时造成了死锁，而如果本线程先收到了 Terminate 消息就不会再去轮询获取消息锁
        for _ in &mut self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("发送终止消息失败");
        }

        println!("Shutting down all workers.");

        // 这里使用了 &mut 是因为 self 本身是一个可变引用而且也需要能够修改 worker
        for worker in &mut self.workers {
            // Option<T>.take() 会将T取出而留下None，所以take()后面不能再链式调用
            if let Some(thread) = worker.thread.take() {
                println!("Shutting down worker {}", worker.id);

                thread
                    .join()
                    .expect(format!("等待线程 {} 执行完毕失败", worker.id).as_str());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn negative_size_new() {
        ThreadPool::new(0);
    }
}
