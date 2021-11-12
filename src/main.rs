fn main() {
    let mut s = String::from("hello world");

    let idx = first_word(&s);

    s = s[0..idx].to_string();

    println!("idx: {}, s: {}", idx, s)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
