#[allow(unused_imports)]
use crate::Config;
use notify::{recommended_watcher, RecursiveMode, Watcher, Event};
use std::{path::Path, sync::mpsc};

pub fn monitor_dir(config: Config) -> () {
    let (tx, rx) = mpsc::channel::<Result<Event, _>>();
    
    let mut watcher = notify::recommended_watcher(tx).unwrap();
    
    watcher.watch(Path::new(&config.filename), RecursiveMode::Recursive);

    for res in rx {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        };
    };
}
