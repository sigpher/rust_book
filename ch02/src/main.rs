use ch02::pi;
// use ch02::{auto_guessing_game, guessing_game};

fn main() {
    // guessing_game(100000);

    // let mut sum = 0;
    // let times = 1000000;
    // let max = 1000000000000;
    // for _ in 1..=times {
    //     let count = auto_guessing_game(max);
    //     sum += count;
    // }
    // println!("范围(0,{max}),测试{times}次,平均猜测次数为{}", sum / times);

    let pi = pi(1000000000);
    println!("{pi}");
}
