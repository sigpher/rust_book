use std::{cmp::Ordering, io};

use rand::Rng;

pub fn guessing_game(max: u64) {
    println!("Guessing Game");
    println!("Please input your guess");
    let secret_number = rand::thread_rng().gen_range(1..=max);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().parse::<u64>().unwrap();

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// 使用二分法自动进行猜数字游戏
pub fn auto_guessing_game(max: u64) -> u64 {
    // println!("Auto Guessing Game");
    let secret_number = rand::thread_rng().gen_range(1..=max);
    let mut guess: u64 = max / 2;
    let mut count_of_guess: u64 = 0;
    let mut l: u64 = 0;
    let mut r: u64 = max;
    // println!("{secret_number}");
    loop {
        count_of_guess += 1;
        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                // println!("Guess: {guess}");
                r = guess;
                guess = (l + r) / 2;
            }
            Ordering::Less => {
                // println!("Guess: {guess}");
                l = guess;
                guess = (l + r) / 2;
            }
            Ordering::Equal => {
                // println!("Guess: {guess}");
                // println!("You win!");
                break;
            }
        }
    }
    // println!("Guess {} times", count_of_guess);
    count_of_guess
}

//使用蒙特卡罗算法对圆周率进行求解
pub fn pi(num: u64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut x: f64;
    let mut y: f64;
    let mut count: u64 = 0;
    for _ in 0..num {
        x = rng.gen();
        y = rng.gen();
        if (x.powi(2) + y.powi(2)) <= 1.0 {
            count += 1;
        }
    }
    4.0 * count as f64 / num as f64
}
