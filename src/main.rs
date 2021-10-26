use std::path::PathBuf;

use inotify::{Inotify, WatchMask};

fn main() {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    let watch_dir = PathBuf::from("/run/dump1090-fa/");

    inotify
        .add_watch(watch_dir, WatchMask::MOVED_TO)
        .expect("Failed to add watch to /run/dump1090-fa/");

    println!("watches set up");

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");
        for event in events {
            println!("Event on {:?}", event.name);
        }
    }
}
