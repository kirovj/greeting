#![allow(unused)]

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 200 {
            panic!("value should between 1, 100")
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn guess_new() {
        Guess::new(200);
    }
}
