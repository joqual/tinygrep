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
    
    for line in search(&cfg.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        // do something with line
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}