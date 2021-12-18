use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, *y);
    // *y => *(y.deref())

    let m = MyBox::new(String::from("jack"));
    // &m &MyBox<String>
    // deref &String
    // deref &str
    hello(&m);
    // hello(&(*m)[..])
    hello("rust");
}
