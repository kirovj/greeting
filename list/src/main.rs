#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Blue")).or_insert(50);

    scores.entry(String::from("red")).or_insert(11);
    println!("{:?}", scores);
}
