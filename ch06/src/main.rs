use ch06::{Coin, IpAddrKind, UsState};

fn main() {
    let v_4 = IpAddrKind::V4(127, 0, 0, 1);
    let v_6 = IpAddrKind::V6("::1");
    println!("{:?}", v_4);
    println!("{:?}", v_6);

    // let some_number = Some(5);
    let some_number: Option<i32> = None;
    if let Some(number) = some_number {
        println!("{}", number);
    }

    let my_coin = Coin::Penny;
    my_coin.value();
    let my_quater = Coin::Quarter(UsState::Alabama);
    let value = my_quater.value();
    println!("{value}");

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other=>move_player(other),
        // _ => reroll(),
        _ => (),
    }
}

pub fn add_fancy_hat() {}
pub fn remove_fancy_hat() {}
pub fn move_player(num_spaces: u8) {}
pub fn reroll() {}
