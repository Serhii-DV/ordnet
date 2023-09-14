use ordnet::Config;
use std::process;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for \"{}\"", config.query);

    if let Err(e) = ordnet::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
