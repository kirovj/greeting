use std::fmt::Debug;
fn compare<T>(left: T, right: T)
where
    T: Debug + PartialEq,
{
    println!("{:?} {} {:?}", left, if left == right { "==" } else { "!=" }, right);
}
fn main() {
    compare("tea", "coffee");
    // prints: "tea" != "coffee"
}