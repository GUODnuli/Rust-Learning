#![allow(unused)]
use trait_learn::{ Summary, Tweet, NewsArticle, notify, returns_summarizable };
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people."),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    // use trait function
    // println!("1 new tweet: {}", tweet.summarize());
    // println!("New article available! {}", article.summarize());
    // use trait argument
    // notify(article);
    // notify(tweet);
    let tweet2 = returns_summarizable();
    println!("{}", tweet2.summarize());
}
