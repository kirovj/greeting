#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Bule"), String::from("Yellow")];

    let init_scores = vec![10, 50];

    let score: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
}
