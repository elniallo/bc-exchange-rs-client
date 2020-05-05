use serde::{Serialize, Deserialize};
use serde_json::to_string;

#[derive(Deserialize, Serialize, Debug)]
pub struct Subscribe {
    action: String,
    channel: String
}

impl Subscribe {
    pub fn new(channel: &str) -> Self {
        Subscribe {
            action: String::from("subscribe"),
            channel: String::from(channel)
        }
    }

    pub fn to_json(&self) -> String{
        serde_json::to_string(&self).unwrap()
    }
}
