use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args = env::args();

    // unwrap_or_else: ok and get result, err then do func
    // |err| close func
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("parse args err: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("App run err: {}", e);
        process::exit(1);
    }
}
