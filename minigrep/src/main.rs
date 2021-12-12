use minigrep::Config;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else: ok and get result, err then do func
    // |err| close func
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("parse args err: {}", err);
        process::exit(1)
    });

    // expect: panic
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
