struct Post {
    content: String,
}

struct DraftPost {
    content: String,
}

struct PendingReviewPost {
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

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Use self here so the PendingReviewPost takes ownership of
    // content and the reference to DraftPost can be dropped
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    // Use self here sp the Post takes ownership of content
    // and the reference to PendingReviewPost can be dropped
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

pub fn exercise() {
    let mut post = Post::new();
    println!("Created draft post");
    post.add_text("Why you should learn Rust?");

    // purposeful shadowing of post. The reference to the
    // previous type is replaced by the calls to request_review
    // and approve
    let post = post.request_review();
    println!("Requested review");
    let post = post.approve();
    println!("Review apporved: {}", post.content());

    assert_eq!("Why you should learn Rust?", post.content());
}
