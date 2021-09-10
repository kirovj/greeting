
fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}
fn main() {
    println!("{}", "Hello there!");
}
// fn main() {
//     use std::io::{self, Write};
//     io::stdout().lock().write_all(b"Hello there!\n").unwrap();
// }