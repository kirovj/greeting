#![allow(unused)]
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Map;
use std::thread;
use std::time::Duration;

// F closure, K, V
struct Cacher<F, K, V>
where
    F: Fn(K) -> V,
{
    func: F,
    map: HashMap<K, V>,
}

impl<F, K, V> Cacher<F, K, V>
where
    F: Fn(K) -> V,
    K: Hash + Eq + Copy,
    V: Copy,
{
    fn new(func: F) -> Cacher<F, K, V> {
        let mut map: HashMap<K, V> = HashMap::new();
        Cacher { func, map }
    }

    fn value(&mut self, key: K) -> V {
        let value = self.map.get(&key);
        match value {
            Some(value) => *value,
            None => {
                let value = (self.func)(key);
                self.map.insert(key, value);
                value
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut calculate = Cacher::new(|s: u32| -> u32 { s + 5 });
    if random_number < 7 {
        println!("Today, do {} pushups!", calculate.value(intensity));
        println!("Next, do {} situps!", calculate.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", calculate.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
