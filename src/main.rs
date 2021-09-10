struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

fn main() {
    let mut n = Number {
        odd: true,
        value: 17
    };
    n.value = 19; // all good
}