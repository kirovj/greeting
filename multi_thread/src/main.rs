use std::{sync::mpsc, thread};

fn main() {
    let (s, r) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("asd");
        s.send(val).unwrap();
    });

    let val = r.recv().unwrap();
    println!("recv: {}", val);
}
