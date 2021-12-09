use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("search for {}", query);
    println!("in the file {}", filename);
}
