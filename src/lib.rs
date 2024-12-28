use std::process;

pub mod read;
pub mod write;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub command: Command,
    pub operation: Option<Operation>,
    pub input: Option<String>,
}

#[derive(Debug)]
pub enum Command {
    Read,
    Write,
    List,
}

#[derive(Debug)]
pub enum Operation {
    At_Line,
    Replace,
    Prepend,
    Append,
}

impl Config {
    pub fn new<'a>(arg: Vec<String>) -> Result<Config, ()> {
        if arg.len() < 3 || arg.len() == 4 {
            // If the inputs are of invalid length
            eprintln!(
                "Please enter a valid command. You have provided --> {:?}",
                arg
            );
            process::exit(0);
        } else {
            // If the user wants to read a file
            let filename: String = arg[1].clone();
            let command: String = arg[2].clone();
            // let operation: String = arg[3].clone();
            // let input: String = arg[4].clone();

            if command == "read" {
                return Ok(Config {
                    filename,
                    command: Command::Read,
                    operation: None,
                    input: None,
                });
            } else if command == "write" {
                let operation: String = arg[3].clone();
                let input: String = arg[4].clone();

                if operation == "replace" {
                    return Ok(Config {
                        filename,
                        command: Command::Write,
                        operation: Some(Operation::Replace),
                        input: Some(input),
                    });
                } else if operation == "prepend" {
                    return Ok(Config {
                        filename,
                        command: Command::Write,
                        operation: Some(Operation::Prepend),
                        input: Some(input),
                    });
                } else if operation == "append" {
                    return Ok(Config {
                        filename,
                        command: Command::Write,
                        operation: Some(Operation::Append),
                        input: Some(input),
                    });
                } else {
                    return Err(println!("Kindly enter a valid operation."));
                }

            } else {
                return Ok(Config {
                    filename,
                    command: Command::List,
                    operation: None,
                    input: None,
                });
            }
        }
    }
}

// To destructure the arg vec to struct
pub fn dest_arg(arg: Vec<String>) -> Config {
    let config: Config = match Config::new(arg) {
        Ok(val) => val,
        Err(_) => {
            process::exit(0);
        }
    };

    config
}
