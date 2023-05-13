// pub fn is_prime(number: u32) -> bool {
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

pub fn is_prime(number: u32) -> bool {
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
