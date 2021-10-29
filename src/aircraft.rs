use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct AircraftFile {
    pub now: f64,
    pub aircraft: Vec<Value>,
}
