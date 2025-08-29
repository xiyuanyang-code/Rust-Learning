use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String {
        format!("READ more from {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // 这里我们只实现了 summarize_author
    // summarize 会使用 trait 的默认实现
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    // 这里我们只实现了 summarize_author
    // summarize 会使用 trait 的默认实现
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.username)
    }
}

pub struct EmptyPost {
    pub username: String,
}

// 这里我们只实现了 summarize_author
// summarize 会使用 trait 的默认实现
impl Summary for EmptyPost {
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

// pub fn notify_new(item_1: &impl Summary, item_2: &impl Summary){}
// pub fn notify_new_2<T: Summary>(item_1: &T, item_2: &T){}

// pub fn notify_double(item: &(impl Summary + Display)){}
// pub fn notify_double_2<T: Summary + Display>(item: &T){}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

fn main() {
    println!("Hello world!");

    let news_1 = NewsArticle {
        headline: String::from("It is a headline"),
        location: String::from("China"),
        author: String::from("Xiyuan Yang"),
        content: String::from("It is a content"),
    };
    println!("{}", news_1.summarize()); // 输出: READ more from Author: Xiyuan Yang...

    let empty_news = EmptyPost {
        username: String::from("It is an empty post"),
    };
    println!("{}", empty_news.summarize()); // 输出: READ more from Author: It is an empty post...

    // using trait as a parameter
    notify(&news_1);
    let s: &'static str = "I have a static lifetime.";
}
