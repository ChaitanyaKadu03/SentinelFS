use std::process;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub command: String,
    pub operation: Option<String>,
    pub input: Option<String>,
}

impl Config {
    pub fn new<'a>(arg: Vec<String>) -> Result<Config, &'a str> {
        if arg.len() < 3 || arg.len() == 4 {
            // If the inputs are of invalid length
            eprintln!(
                "Please enter a valid command. You have provided --> {:?}",
                arg
            );
            process::exit(0);
        } else if arg.len() == 3 {
            // If the user wants to read a file
            let filename: String = arg[1].clone();
            let command: String = arg[2].clone();

            return Ok(Config {
                filename,
                command,
                operation: None,
                input: None,
            });
        } else {
            // If the user wants to read a file particular line
            let filename: String = arg[1].clone();
            let command: String = arg[2].clone();
            let operation: String = arg[3].clone();
            let input: String = arg[4].clone();

            return Ok(Config {
                filename,
                command,
                operation: Some(operation),
                input: Some(input),
            });
        }
    }
}

// To destructure the arg vec to struct
pub fn dest_arg(arg: Vec<String>) -> Config {
    let config: Config = match Config::new(arg) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Kindly enter valid command. Error is --> {}", err);
            process::exit(0);
        }
    };

    config
}
