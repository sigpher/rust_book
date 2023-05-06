use crate::lifetime::longest;
use ch10::{notify, NewsArticle, Tweet};

pub mod lifetime;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
        ),
    };

    let msg: String = notify(&tweet);
    println!("{msg}");
    let msg = notify(&article);
    println!("{msg}");
    let r = 10i32;
    println!("r:{}", r);

    let s1 = "abcd";
    let s2 = "hi";
    let longest = longest::longest(s1, s2);
    println!("{}",longest);

    let result =longest::longest_with_announcement("abcd","xyz","Today is someone's birthday!");
    println!("The longest string is {}",result);
}
