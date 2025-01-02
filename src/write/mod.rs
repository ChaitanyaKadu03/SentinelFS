use super::{Config, Operation};
use std::fs::{read_to_string, write};

// This part is reponsible for writing to the file
// Takes config as input => Expects all parameters of config to exist.
//
pub fn write_file(config: Config) -> () {
    // Checks the opperation
    match &config.operation {
        // If the opp value is not none
        Some(opp) => match opp {
            // Matches the opperation
            Operation::Replace => match config.input {
                Some(content) => {
                    match write(&config.filename, content) {
                        Ok(_) => {
                            println!("The file is updated successfully!. You can read it using cargo run {} read", &config.filename)
                        }
                        Err(err) => {
                            println!("An issue occured. {}", err)
                        }
                    };
                }
                None => {
                    println!("Kindly provide some content. The inputs were {:#?}", config)
                }
            },
            // Matches the opperation
            Operation::Prepend => match &config.input {
                Some(content) => {
                    let prior_content: String = match read_to_string(&config.filename) {
                        Ok(text) => text,
                        Err(err) => {
                            println!("An error occured. Error --> {}", err);
                            "".to_string()
                        }
                    };

                    match write(&config.filename, format!("{}{}", content, prior_content)) {
                        Ok(_) => {
                            println!("The file is updated successfully!. You can read it using cargo run {} read", &config.filename)
                        }
                        Err(err) => {
                            println!("An issue occured. {}", err)
                        }
                    };
                }
                None => {
                    println!("Kindly provide some content. The inputs were {:#?}", config)
                }
            },
            // Matches the opperation
            Operation::Append => match config.input {
                Some(content) => {
                    let prior_content: String = match read_to_string(&config.filename) {
                        Ok(text) => text,
                        Err(err) => {
                            println!("An error occured. Error --> {}", err);
                            "".to_string()
                        }
                    };
                    match write(&config.filename, format!("{}{}", prior_content, content)) {
                        Ok(_) => {
                            println!("The file is updated successfully!. You can read it using cargo run {} read", &config.filename)
                        }
                        Err(err) => {
                            println!("An issue occured. {}", err)
                        }
                    };
                }
                None => {
                    println!("Kindly provide some content. The inputs were {:#?}", config)
                }
            },
            Operation::__ => {
                //
            }
        },
        None => {
            //
        }
    }
}
