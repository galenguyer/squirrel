use inotify::{Inotify, WatchMask};
use mongodb::bson::Document;
use serde_json::json;
use std::{env, fs::read_to_string, path::PathBuf};
mod aircraft;

fn main() {
    let uri = env::var("SQUIRREL_DB").expect("SQUIRREL_DB not set");
    let client = mongodb::sync::Client::with_uri_str(uri);
    match client {
        Err(_) => {
            panic!("Failed to connect to MongoDB");
        }
        _ => {}
    }
    let database = client.unwrap().database("dump1090");
    let collection = database.collection::<Document>("aircraft");

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
                                let file_contents: aircraft::AircraftFile =
                                    serde_json::from_str(&contents).unwrap();
                                match collection.insert_many(
                                    file_contents
                                        .aircraft
                                        .iter()
                                        .map(|x| {
                                            let mut a = x.clone();
                                            a["now"] = json!(file_contents.now);
                                            mongodb::bson::to_document(&a).unwrap()
                                        })
                                        .collect::<Vec<Document>>(),
                                    None,
                                ) {
                                    Err(err) => {
                                        println!(
                                            "Error inserting aircraft into database: {:?}",
                                            err
                                        );
                                    }
                                    Ok(res) => {
                                        println!(
                                            "Inserted {} documents at {}",
                                            res.inserted_ids.len(),
                                            file_contents.now
                                        )
                                    }
                                }
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
