
fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![true, false, true];
    print_type_name(&v1); // prints "Vec<i32>"
    print_type_name(&v2); // prints "Vec<bool>"
}