use std::env;
use std::process;

use tinygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg: Config = match Config::build(&args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Problem parsing arguments: {e}");
            process::exit(1);
        }
    };

    if let Err(e) = tinygrep::run(cfg) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
