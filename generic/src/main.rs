fn find_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn main() {
    let list = vec![10, 34, 23];
    print!("largest: {}", find_largest(&list))
}
