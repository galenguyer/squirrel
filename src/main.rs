use std::{fs::read_to_string, path::PathBuf};
use inotify::{Inotify, WatchMask};

mod aircraft;

fn main() {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    let watch_dir = PathBuf::from("/run/dump1090-fa/");
    let aircraft_file = PathBuf::from("/run/dump1090-fa/aircraft.json");

    inotify
        .add_watch(watch_dir, WatchMask::MOVED_TO)
        .expect("Failed to add watch to /run/dump1090-fa/");

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");
        for event in events {
            match event.name {
                Some(path) => {
                    if path.eq("aircraft.json") {
                        match read_to_string(&aircraft_file) {
                            Ok(contents) => {
                                let file_contents: aircraft::AircraftFile = serde_json::from_str(&contents).unwrap();
                                println!("now: {}, aircraft: {}", file_contents.now, file_contents.aircraft.len());
                            }
                            Err(_) => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
