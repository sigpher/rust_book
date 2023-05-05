fn main() {
    let s1 = String::from("hello world");
    println!("'{}'", first_word(&s1));
}

fn first_word(s: &str) -> &str {
    let bytes = s.trim().as_bytes();
    for (idx, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..idx];
        }
    }
    return &s[..];
}


fn find_number(numbers:&[u8],num:u8)->usize{
    for (idx,&item) in numbers.iter().enumerate(){
        if item == num{
            return idx
        }
    }
    return 0
}