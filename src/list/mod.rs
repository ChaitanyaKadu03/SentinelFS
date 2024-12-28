use super::Config;
use std::fs;

pub fn list_file(config: Config) {
    match fs::read_dir(&config.filename) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let file_name = entry.file_name();

                        println!("File name: {}", file_name.to_string_lossy());
                    }
                    Err(err) => {
                        eprintln!("Error reading entry: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to read directory '{}': {}", config.filename, err);
        }
    }
}
