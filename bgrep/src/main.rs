use std::process;
use bgrep::Config;
fn main() {
    //Note that args[0] contains the programs name
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
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

