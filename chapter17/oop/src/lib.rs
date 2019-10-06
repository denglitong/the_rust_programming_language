pub struct MyPost {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl MyPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

// 将动作和不同阶段的类型区分开，减少重复代码，也避免出错
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 利用 Rust 的所有权转移，可以在不同阶段的类型中移交所有权
    // 在 Rust 中面向对象设计模式并不总是最好的解决方案，
    // 因为 Rust 拥有像所有权这样的面向对象对象语言所没有的功能
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> MyPost {
        MyPost {
            content: self.content,
        }
    }
}
