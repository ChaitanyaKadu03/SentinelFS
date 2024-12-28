use sentinel_fs::*;
use std::env;
use sentinel_fs::read::read_file;
use sentinel_fs::write::write_file;

fn main() {
    let arg: Vec<String> = env::args().collect();

    let config: Config = dest_arg(arg);

    match &config.command {
        &Command::Read => {
            read_file(config);
        },
        &Command::Write => {
            write_file(config);
        },
        &Command::List => {
            read_file(config);
        }
    }

}
