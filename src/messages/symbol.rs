use serde::{Serialize, Deserialize};
use serde_json::to_string;

#[derive(Deserialize, Serialize, Debug)]
pub struct SymbolSubscription {
    action: String,
    channel: String,
    symbol: String
}

impl SymbolSubscription {
    pub fn new(channel: &str, symbol: Symbol) -> Self {

        SymbolSubscription {
            action: String::from("subscribe"),
            channel: String::from(channel),
            symbol: symbol.as_pair()
        }
    }

    pub fn to_json(&self) -> String{
        serde_json::to_string(&self).unwrap()
    }
}

pub struct Symbol<'a> {
    base: &'a str,
    counter: &'a str
}

impl <'a> Symbol <'a> {
    pub fn new(base: &'a str, counter: &'a str) -> Self {
        Self {
            base: base,
            counter: counter
        }
    }

    pub fn as_pair(&self) -> String {
        let mut pair = String::default();
        pair.push_str(self.base);
        pair.push('-');
        pair.push_str(self.counter);
        pair
    }

}