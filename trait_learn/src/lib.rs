#![allow(unused)]
use std::fmt;
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
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

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait bound
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// the short type
// pub fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// more struct argument
// pub fn notify<T: Summary>(item1: T, item2: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// more trait bound with "+"
// pub fn notify<item: impl Summary + Display> { }

// more trait bound with where
// pub fn notify<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 { }

// pub fn notify<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//     U: Clone + Debug
// { }

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// cant's complie
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best
//             hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }