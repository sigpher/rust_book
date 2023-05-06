

use std::fmt::Display;

pub fn get_largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];
    list.iter().for_each(|&num| {
        if num > largest {
            largest = num
        }
    });
    largest
}

pub trait Summary {
    fn summarize(&self) -> String;
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

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

// pub fn notify(item:&impl Summary)->String{
//     format!("{}",item.summarize())
// }
// pub fn notify<T:Summary>(item:&T)->String{
//     format!("{}", item.summarize())
// }
// pub fn notify<T>(item: &T) -> String
// where
//     T: Summary,
// {
//     format!("{}", item.summarize())
// }

pub fn notify<T: Summary + Display>(item: &T) -> String {
    print!("{}:\t--", item);
    format!("{}", item.summarize())
}
