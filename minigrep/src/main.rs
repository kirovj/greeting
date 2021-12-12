use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else: ok and get result, err then do func
    // |err| close func
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("parse args err: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("App run err: {}", e);
        process::exit(1);
    }
}
