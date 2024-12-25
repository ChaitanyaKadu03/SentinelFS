use sentinel_fs::*;
use std::env;

fn main() {
    let arg: Vec<String> = env::args().collect();

    let config: Config = dest_arg(arg);

    println!("The config is - {:?}", config);
}
