use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num += 5;
    }

    println!("m = {:?}", m);
}
