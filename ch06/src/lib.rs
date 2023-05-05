#[derive(Debug)]
pub enum IpAddrKind<'a> {
    V4(u8, u8, u8, u8),
    V6(&'a str),
}

#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    pub fn value(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Satate quater from {:?}", state);
                25
            }
        }
    }
}

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn add_one(x:Option<i32>)->i32{
    match x{
        Some(i) => i,
        None => 0,
    }
}

// use Option to implement default parameter;
pub fn username(s:Option<String>)->String{
    if let None = s{
        return "choi".into();
    }
    s.unwrap()
}



