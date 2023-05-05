use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
    println!("--------------------");
    scores.insert(String::from("Yellow"), 100);
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    let entry = scores
        .entry(String::from("Blue"))
        // .and_modify(|e| *e += 10000)
        .or_insert_with(|| 0);
    // .or_insert(0);
    println!("{:?}", entry);

    println!("--------------------");
    let text = "how do you do";
    let m = word_counter(text);
    println!("{:?}", m);
    println!("--------------------");
    let m = char_counter(text);
    println!("{:?}", m);
}

fn word_counter(text: &str) -> HashMap<&str, u8> {
    let mut map = HashMap::new();
    // for word in text.split_whitespace(){
    //     let count = map.entry(word.to_string()).or_insert(0);
    //     *count +=1;
    // }

    text.split_whitespace().for_each(|w| {
        map.entry(w).and_modify(|e| *e += 1).or_insert(1);
    });

    map
}

fn char_counter(text: &str) -> HashMap<char, u8> {
    let mut map = HashMap::new();
    text.chars().for_each(|ch| {
        map.entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });
    map
}
