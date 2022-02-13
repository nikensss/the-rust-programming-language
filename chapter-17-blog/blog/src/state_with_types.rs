pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
            required_approvals: 2,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approvals: u32,
    required_approvals: u32,
}

impl PendingReviewPost {
    pub fn approve(&mut self) {
        self.approvals += 1;
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn is_approved(&self) -> bool {
        self.approvals == self.required_approvals
    }

    pub fn get_approvals(&self) -> u32 {
        self.approvals
    }

    pub fn get_post(self) -> Option<Post> {
        if self.approvals == self.required_approvals {
            return Some(Post {
                content: self.content,
            });
        }
        None
    }

    pub fn get_required_approvals(&self) -> u32 {
        self.required_approvals
    }

    pub fn set_required_approvals(&mut self, required_approvals: u32) {
        self.required_approvals = required_approvals
    }
}

pub fn pseudo_main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let mut post = post.request_review();
    post.set_required_approvals(4);
    assert_eq!(4, post.get_required_approvals());

    while !post.is_approved() {
        post.approve();
    }

    assert_eq!(post.get_approvals(), post.get_required_approvals());
    if let Some(p) = post.get_post() {
        assert_eq!("I ate a salad for lunch today", p.content());
    } else {
        panic!("It should've been approved!");
    }
}
