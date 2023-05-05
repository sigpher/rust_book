fn main() {
    // let x = 5;
    let mut x = 5;
    println!("x: {}", x);
    println!("The value of x is: {}", x);
    x = 6;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("3 hours in seconds is: {THREE_HOURS_IN_SECONDS}s");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The spaces is {spaces}");

    let guess: u32 = "0o42".parse().unwrap_or_default();
    println!("The guess is {guess}");

    let f1 = 2f64;
    let f2 = 3f32;

    let tup = (500, 6.4, true);

    let arr = [1, 2, 3, 4, 5, 6];

    let giveback = take_and_giveback(10);
    println!("givenback: {}",giveback);

}

fn take_and_giveback(x: u8) -> u8 {
    println!("{x}");
    x
}
