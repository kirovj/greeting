struct Number {
    odd: bool,
    value: i32,
}

// the `Neg` trait is used to overload `-`, the
// unary minus operator.
impl std::ops::Neg for Number {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            value: -self.value,
            odd: self.odd,
        }        
    }
}

fn print_i32(x: i32) {
    println!("x = {}", x);
}
fn print_number(n: Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}
fn main() {
    let n = Number { odd: true, value: 51 };
    print_number(n); // `n` is moved
    print_number(n); // error: use of moved value: `n`
}