use ch13::{Inventory, ShirtColor};
use rand::{self, Rng};
use std::thread;

fn main() {
    let store: Inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };
    let user_pref1 = Some(ShirtColor::Blue);
    let getaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, getaway1
    );
    let user_pref2 = None;
    let getaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, getaway2
    );
    println!("-------------------");

    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // let only_borrows = || println!("From closure: {:?}", list);
    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);

    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // let mut borrows_mutably = || list.push(7);
    // borrows_mutably();
    // println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    let list = [2, 3, 4];

    let random_number = rand::thread_rng().gen_range(0..list.len());

    let number = Exist::Yes(10);
    let unwrap_number = number.yes_or_else(|| list[random_number]);
    let number = Exist::No;
    let unwrap_number = number.yes_or_else(|| list[random_number]);
    println!("{unwrap_number}");
    println!("-------------------");
    let mut rects = vec![
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    // println!("-------------------");
    // println!("{:#?}", rects);
    // rects.sort_by_key(|r| r.width);
    // println!("-------------------");
    println!("{:#?}", rects);

    // rects.sort_by_key(|r| r.height);
    rects.sort_by(|&a, &b| a.height.cmp(&b.height));
    println!("-------------------");
    println!("{:#?}", rects);
}

enum Exist<T> {
    Yes(T),
    No,
}

impl<T> Exist<T> {
    pub fn yes_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Exist::Yes(x) => x,
            Exist::No => f(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rectangle<T> {
    width: T,
    height: T,
}
