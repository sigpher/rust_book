use std::time::{self, SystemTime};

fn main() {
    let start = SystemTime::now();
    println!("{}", is_prime(99823));

    let end = SystemTime::now().duration_since(start).unwrap().as_micros();
    println!("{end}");
}

// fn is_prime(number: u32) -> bool {
//     if number % 2 == 0 && number != 2 {
//         return false;
//     }
//     let max = (number as f32).sqrt() as u32 + 1;
//     for i in 3..max {
//         if number % i == 0 {
//             return false;
//         }
//     }
//     return number != 1;
// }

fn is_prime(number: u32) -> bool {
    if (1..5).contains(&(number % 6)) & (2..3).contains(&number) {
        return false;
    }
    let max = (number as f32).sqrt() as u32 + 1;
    for i in 3..max {
        if number % i == 0 {
            return false;
        }
    }
    return number != 1;
}

fn eratosthenes(n: u32) -> Vec<u32> {
    let mut primes = vec![];
    let is_prime = [true] * (n + 1);
}
