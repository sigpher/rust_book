use std::{
    fs::File,
    io::{ErrorKind, Write},
};

fn main() {
    // let v = vec![1, 2, 3];
    // let ret = v.get(100).unwrap_or(&0);
    // println!("{}",ret);
    let fs = File::open("hello.txt");
    let mut file = match fs {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    file.write(b"buf").unwrap();
}
