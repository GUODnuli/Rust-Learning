use std::{ env, process };

use minigrep::Config;

fn main() {
    // Update: 改进之前的I/O项目
    // let args: Vec<String> = env::args().collect();
    
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}