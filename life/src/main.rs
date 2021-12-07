fn main() {
    let r;
    {
        let x = 6;
        r = &x;
    }
    println!("r: {}", r)
}
