use std::borrow::{Borrow, BorrowMut};

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state
            .as_ref()
            .unwrap()
            .add_text(self.content.borrow_mut(), text);
    }

    pub fn content(&self) -> &str {
        // 此处使用 as_ref 是希望获得引用而不是获取其所有权，
        // 如果获取所有权则会因为不能将 state 从 immutable self 中 move
        // 此处使用 unwrap 而不考虑 None 是因为我们在上下文中比编译器知道的更多，
        // 这里 Post 所有的方法都保证了返回的实例中 state 有一个 Some(s) 值
        self.state.as_ref().unwrap().content(&self)
    }

    //    pub fn state(&self) -> &Box<dyn State> {
    //        self.state.unwrap().borrow()
    //    }

    pub fn request_review(&mut self) {
        // 调用 Option.take 方法将值取出并留下一个 None，这使得这里是 move 而不是 borrow
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

// 发布博文的工作流的规则实现了状态模式，
// 围绕这些规则的逻辑都存在于转态对象中而不是分散在 Post 之中

trait State {
    // self: Box<Self> 该参数不同于 self, &self, &mut self，意味着这个方法调用只对这个类型的 Box 有效
    // 这个语法获取了 Box<Self> 的所有权，使老状态无效化以便 Post 的状态值可以将自身转换为新状态
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    // 根据状态来觉得返回内容，默认返回空字符串，然后在 Publish 中覆盖这个方法返回内容
    // 因为该函数返回参数是形参的引用，所以需要声明生命周期参数
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn add_text(&self, content: &mut String, text: &str) {}
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {
            publish_ensure_count: 0,
        })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, content: &mut String, text: &str) {
        content.push_str(text);
    }
}

struct PendingReview {
    publish_ensure_count: i32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if let 2 = self.publish_ensure_count + 1 {
            Box::new(Published {})
        } else {
            Box::new(PendingReview {
                publish_ensure_count: self.publish_ensure_count + 1,
            })
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
