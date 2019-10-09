pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    /*
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    */
}

pub fn notify(item: impl Summary, item2: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: T, item2: T) {
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::Debug;
use std::fmt::Display;

pub fn notify3(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify4<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    32
}

pub fn some_function2<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    32
}

pub fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
