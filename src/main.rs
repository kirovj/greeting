
fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}
fn main() {
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine
    let o2: Option<i32> = None;
    o2.unwrap(); // this panics!
}
// output: thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:378:21
