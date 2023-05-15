use std::error::Error;

pub struct Config {
    pub query: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();

        Ok(Config { query })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Running with:\n{}", config.query);
    Ok(())
}
