fn print<T: Display>(value: T) {
    println!("value = {}", value);
}
fn print<T: Debug>(value: T) {
    println!("value = {:?}", value);
}