pub mod state_with_types;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Default for Post {
    fn default() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}
impl Post {
    pub fn new() -> Post {
        Post::default()
    }

    pub fn add_text(&mut self, text: &str) {
        let text = self.state.as_ref().unwrap().add_text(text);
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
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
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn add_text<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        text
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }
}

struct PendingReview {
    approvals: u32,
}

impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approvals == 0 {
            return Box::new(PendingReview { approvals: 1 });
        }
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
