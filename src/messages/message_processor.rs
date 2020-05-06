use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use serde_json::Value;
use std::sync::Arc;
use std::sync::Mutex;

pub struct JsonMessageProcessor {
    tx: mpsc::Sender<serde_json::Map<String, serde_json::Value>>,
    rx: mpsc::Receiver<String>,
    authenticated: bool
}

impl JsonMessageProcessor {
    pub fn new(tx: mpsc::Sender<serde_json::Map<String, serde_json::Value>>, rx: mpsc::Receiver<String>) -> Self {
        Self {
            tx,
            rx,
            authenticated: false
        }
    }

    pub fn run(&mut self, orders: Arc<Mutex<Vec<u64>>>) {
        loop {
            match self.rx.try_recv() {
                Ok(data) => {
                    let parsed = JsonMessageProcessor::parse_message(&data).as_object().unwrap().clone();

                    let default_value = Value::String(String::default());
                    let channel = match parsed.get("channel") {
                        Some(msg) => msg,
                        None => &default_value
                    };
                    let event = match parsed.get("event") {
                        Some(msg) => msg,
                        None => &default_value
                    };
                    if channel.as_str() == Some("auth") && event.as_str() == Some("subscribed") {
                        println!("Successful auth response");
                        self.authenticated = true;
                        self.tx.send(parsed).unwrap();
                        continue;
                    }
                    if (channel.as_str() == Some("trading") || channel.as_str() == Some("balances")) && self.authenticated {
                        println!("Trade or balance update");
                        self.tx.send(parsed).unwrap();
                        continue;
                    }
                },
                Err(e) => match e {
                    TryRecvError::Disconnected => break,
                    _ => {}
                    }
                }
            }
    }

    fn parse_message(message: &str) -> Value {
        serde_json::from_str(message).unwrap()
    }
}