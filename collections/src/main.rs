use std::collections::HashMap;

fn main() {

    println!("Vector:");

    let mut v = vec![100, 32, 57];

    v.push(1);

    for i in &v {
        println!("{}", i);
    }

    println!("Hash Map:");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


}
