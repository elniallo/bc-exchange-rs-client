use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Authenticate {
    action: String,
    channel: String,
    token: String
}

impl Authenticate {
    pub fn new(api_key: &str) -> Self {
        Authenticate {
            action: String::from("subscribe"),
            channel: String::from("auth"),
            token: String::from(api_key)
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}