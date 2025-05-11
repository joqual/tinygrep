use std::env;
use std::process;

use tinygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = match Config::build(&args) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Problem parsing arguments: {e}");
            process::exit(1);
        }
    };
    
    println!("Searching for {} in file {} ...", cfg.query, cfg.filepath);

    if let Err(e) = tinygrep::run(cfg) {
        println!("Application error: {e}");
        process::exit(1);
    };
}