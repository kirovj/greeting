#![allow(unused)]

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("value < 1")
        } else if value > 100 {
            panic!("value > 100")
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "100")]
    fn guess_new() {
        Guess::new(200);
    }
}
