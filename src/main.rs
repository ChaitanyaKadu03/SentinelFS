use sentinel_fs::*;
use std::env;
use sentinel_fs::read::read_file;
use sentinel_fs::write::write_file;
use sentinel_fs::list::list_file;
use sentinel_fs::delete::delete_file;

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
            list_file(config);
        },
        &Command::Delete => {
            delete_file(config);
        }
    }

}
