extern crate notify;

use dircpy::*;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering raw events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    let base_dir: String = String::from("/home/msamgan/Documents");
    let target_dir: String = String::from("/home/msamgan/Dropbox");

    watcher.watch(&base_dir, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?} {:?} ({:?})", op, path, cookie);
                CopyBuilder::new(&base_dir, &target_dir)
                    .overwrite_if_newer(true)
                    .overwrite_if_size_differs(true)
                    .run()
                    .unwrap();
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
