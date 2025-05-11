use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();
    
        Ok(Config { query, filepath })
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents= fs::read_to_string(cfg.filepath)?;
    Ok(())
}