pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn exercise() {
    let tweet = Tweet {
        username: String::from("RiseOne"),
        content: String::from("Let's learn some rust today!"),
        reply: false,
        retweet: false,
    };

    print!("New tweet: {}", tweet.summarize());
    notify(&tweet);
    trait_bound_notify(&tweet);
}

// The below two implementations are the same
fn notify(summary: &impl Summary) {
    print!("Notification: {}", summary.summarize())
}

fn trait_bound_notify<T: Summary>(summary: &T) {
    print!("Notification: {}", summary.summarize())
}
