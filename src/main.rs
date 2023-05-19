use ordnet::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for \"{}\"", config.query);

    if let Err(e) = ordnet::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
