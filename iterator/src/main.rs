#[derive(Debug, PartialEq)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let my_shoes = shoes_in_size(shoes, 10);
    println!("{:#?}", my_shoes);
    println!("------------------");
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
