use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let h = thread::spawn(move || {
        println!("list: {:?}", v);
    });

    h.join().unwrap();
}
