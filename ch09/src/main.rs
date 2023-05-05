use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let v = vec![1, 2, 3];
    // let ret = v.get(100).unwrap_or(&0);
    // println!("{}",ret);
    // let fs = File::open("hello.txt");
    // let mut file = match fs {
    //     Ok(f) => f,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };
    // file.write(b"buf").unwrap();
    let text = "how wolrd";
    let last_char = last_char_of_first_line(text);
    println!("{:?}", last_char);
    Ok(())
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
