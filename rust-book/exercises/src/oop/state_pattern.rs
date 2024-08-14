struct Post {
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

    pub fn update_content(&mut self, new_content: &str) {
        self.content = self
            .state
            .as_ref()
            .unwrap()
            .update_content(&self.content, new_content)
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn update_content(&self, current_str: &str, new_content: &str) -> String {
        return String::from(current_str);
    }
}

struct Draft {}

struct PendingReview {}

struct PendingApproval {}

struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn update_content(&self, current_str: &str, new_content: &str) -> String {
        return format!("{}{}", current_str, new_content);
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingApproval {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

impl State for PendingApproval {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

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

pub fn exercise() {
    let mut post = Post::new();

    post.add_text("Why you should learn Rust?");
    println!("Content: {}", post.content());
    assert_eq!("", post.content());

    post.update_content(" Because, it's Awesome!");

    post.request_review();
    println!("Content: {}", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("First approval - Content: {}", post.content());
    assert_eq!("", post.content());

    post.reject();
    println!("Reject approval - Content: {}", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("Approval after rejection - Content: {}", post.content());
    assert_eq!("", post.content());

    post.request_review();
    println!("Content: {}", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!(
        "First approval after rejection - Content: {}",
        post.content()
    );
    assert_eq!("", post.content());

    post.approve();
    println!(
        "Second approval after rejection - Content: {}",
        post.content()
    );
    assert_eq!(
        "Why you should learn Rust? Because, it's Awesome!",
        post.content()
    );
}
