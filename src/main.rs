use std::env;
use std::process;
use notgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err); // print it so we dont get the extra stuff printed from panic!
        process::exit(1); // status 1 is program exited with error
    });

    if let Err(e) = notgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
