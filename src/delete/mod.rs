use super::Config;
use std::fs;

pub fn delete_file(config: Config) {
    match fs::remove_dir(&config.filename) {
        Ok(_) => {
            println!("Deleted Successfully!");
        }
        Err(_) => {
            match fs::remove_file(&config.filename) {
                Ok(_) => println!("Deleted Successfully!", ),
                Err(_) => println!("Failed to delete"),
            };
        }
    };
}
