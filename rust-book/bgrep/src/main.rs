use std::env;
use std::process;
use bgrep::Config;


fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}",err);
        process::exit(1);
    });

    println!("Searching for \'{}\'", config.query);
    println!("In file {}\n", config.filename);
    
    if let Err(e) = bgrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

