#![allow(unused)]
use generic::{notify, Summary, Tweet};

fn find_largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn main() {
    let list = vec![10, 34, 23];
    println!("largest: {}", find_largest(&list));

    let list = vec!['c', 'b', 'e'];
    println!("largest: {}", find_largest(&list));

    let list = vec![String::from("hello"), String::from("world")];
    println!("largest: {}", find_largest(&list));
}
