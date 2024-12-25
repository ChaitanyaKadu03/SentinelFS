use sentinel_fs::*;
use std::env;
use sentinel_fs::read::read_file;

fn main() {
    let arg: Vec<String> = env::args().collect();

    let config: Config = dest_arg(arg);

    read_file(config);

}
