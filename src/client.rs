use websocket::client::ClientBuilder;
use websocket::message::OwnedMessage;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

pub struct Client {
    _api_key: String,
    ws_address: String,
    bc_origin: String
}

impl Client {
    pub fn new(api_key: &str, ws_address: &str) -> Self {
        Self {
            _api_key: String::from(api_key),
            ws_address: String::from(ws_address),
            bc_origin: String::from("https://exchange.blockchain.com")
        }
    }

    pub fn run(&mut self, sink: mpsc::Receiver<String>, send: mpsc::Sender<String>) -> std::thread::JoinHandle<Result<(), websocket::WebSocketError>> {
        let mut client = ClientBuilder::new(&self.ws_address).unwrap()
            .origin(self.bc_origin.clone()).connect(None).unwrap();
        let thread_handler = thread::spawn(move || {
            let client_ref = &mut client;
            loop {
                match sink.try_recv() {
                    Ok(msg) => client_ref.send_message(&OwnedMessage::Text(msg)).unwrap(),
                    Err(e) => match e {
                        TryRecvError::Disconnected => break,
                        _ => {}
                    }
                };
                
                let mut should_break: bool = false;
                client_ref.incoming_messages().take(1).for_each(|message| {
                     match message {
                        Ok(data) => {
                            match data {
                                OwnedMessage::Close(_) => should_break = true,
                                OwnedMessage::Ping(_) => {},
                                OwnedMessage::Text(text) => {
                                    match send.send(text.clone()) {
                                        Ok(_) => { println!("Response: {:?}", &text)},
                                        Err(_) => should_break = true
                                    }
                                },
                                _ => println!("Message: {:?}", data)
                            }
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                            should_break = true;
                        }
                     }
                });
                if should_break {
                    break;
                }
             }
             Ok(())
        });
        return thread_handler;
    }
}