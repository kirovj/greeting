struct Number {
    odd: bool,
    value: i32,
}

trait Signed {
    fn is_strictly_negative(self) -> bool;
}

impl Signed for i32 {
    fn is_strictly_negative(self) -> bool {
        self < 0
    }
}
fn main() {
    let n: i32 = -44;
    println!("{}", n.is_strictly_negative()); // prints "true"
}