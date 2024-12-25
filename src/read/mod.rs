use crate::*;
use std::fs;
use std::process;

pub fn read_file(config: Config) -> () {
 
    let text: String = match fs::read_to_string(config.filename) {
    Ok(txt) => txt,
    Err(err) => {
        eprintln!("Failed to print. Error is -> {}", err);
        process::exit(0);
    }
    };

    println!("{}", text);

}
