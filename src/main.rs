fn main() {
    let s = String::from("hello world");

    let first_world = first_word(&s);

    println!("first world: {}", first_world)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
