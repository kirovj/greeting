use std::{thread, time::Duration};

fn main() {
    let h = thread::spawn(|| {
        for i in 1..10 {
            println!("no: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main no: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    h.join().unwrap();
}
