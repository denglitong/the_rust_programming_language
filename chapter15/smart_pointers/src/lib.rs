pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger
                .send("Warning: You've used up 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger
                .send("Urgent warning: Your've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: Your are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LimitTracker, Messenger};
    use std::cell::RefCell;

    struct MockMessenger {
        // RefCell<T> 在运行时记录借用
        // 当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法，
        // 对于 RefCell<T> 来说则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全API的一部分
        // RefCell 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针，
        // 每次调用 borrow 则 RefCell<T> 将 Ref<T> 的引用计数加一，当 Ref<T> 离开作用域时减一
        // RefCell<T> 在任何时刻只允许 一个可变的借用 或者是 多个不可变的借用（即读/写的scope要隔离）
        // 如果违反上述规则，RefCell<T> 的实现会在运行时 panic!
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // borrow_mut 方法返回 RefMut 类型的智能指针
            // Ref 和 RefMut 都实现了 Deref trait 所以可以当做常规引用对待
            self.send_messages.borrow_mut().push(String::from(msg));

            //let mut one_borrow = self.send_messages.borrow_mut();
            //let mut two_borrow = self.send_messages.borrow_mut();
            //one_borrow.push(String::from(msg));
            //two_borrow.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // borrow 方法返回 Ref 类型的智能指针
        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
    }
}
