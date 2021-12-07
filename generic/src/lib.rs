use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

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
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary /* + Display */) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary /* + Display */>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T: Display + Clone, U: Clone + Summary>(t: T, u: U) -> i32 {
    todo!()
}

fn some_function1<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    todo!()
}
