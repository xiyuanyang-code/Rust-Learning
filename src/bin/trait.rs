pub trait Summary {
    fn summarize(&self) -> String {
        String::from("READ More(...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct EmptyPost {
    pub username: String,
}

// empty trait
// using the default method
impl Summary for EmptyPost {}

fn main() {
    println!("Hello world!");
    let news_1 = NewsArticle {
        headline: String::from("It is a headline"),
        location: String::from("China"),
        author: String::from("Xiyuan Yang"),
        content: String::from("It is a content"),
    };
    println!("{}", news_1.summarize());

    let empty_news = EmptyPost {
        username: String::from("It is an empty post"),
    };
    println!("{}", empty_news.summarize());
}
