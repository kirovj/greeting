struct Number {
    odd: bool,
    value: i32,
}

fn invert(n: &mut Number) {
    n.value = -n.value;
}
fn print_number(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}
fn main() {
    // this time, `n` is mutable
    let mut n = Number { odd: true, value: 51 };
    print_number(&n);
    invert(&mut n); // `n is borrowed mutably - everything is explicit
    print_number(&n);
}