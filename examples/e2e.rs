use blockchain_exchange_client::client;
use blockchain_exchange_client::messages;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let api_key = String::from("YOUR API KEY HERE");
    let ws_address = "wss://ws.prod.blockchain.info/mercury-gateway/v1/ws";
    let (tx, rx) = channel::<String>();
    // Channel for sending to socket
    let (json_send, json_recv) = channel::<String>();

    //Channel for recieving JSON map of authenticated responses
    let (auth_send, auth_recv) = channel::<serde_json::Map<String, serde_json::Value>>();
    let mut client = client::Client::new(&api_key, ws_address);
    let client_handler = client.run(rx, json_send);

    let subscribe_message = messages::subscribe::Subscribe::new("heartbeat");
    println!("Message: {:?}", subscribe_message);
    match tx.send(subscribe_message.to_json()) {
        Ok(_) => println!("Sent message"),
        Err(_) => println!("Error")
    }

    let l2 = messages::symbol::SymbolSubscription::new("l2", messages::symbol::Symbol::new("BTC", "USD"));
        match tx.send(l2.to_json()) {
        Ok(_) => println!("Sent L2 message"),
        Err(_) => println!("Error")
    }

    let l3 = messages::symbol::SymbolSubscription::new("l3", messages::symbol::Symbol::new("BTC", "USD"));
    match tx.send(l3.to_json()) {
        Ok(_) => println!("Sent L3 message"),
        Err(_) => println!("Error")
    }

    let symbols = messages::subscribe::Subscribe::new("symbols");
    println!("Message: {:?}", symbols);
    match tx.send(symbols.to_json()) {
        Ok(_) => println!("Sent message"),
        Err(_) => println!("Error")
    }

    let trades = messages::symbol::SymbolSubscription::new("trades", messages::symbol::Symbol::new("ETH", "USD"));
    match tx.send(trades.to_json()) {
        Ok(_) => println!("Sent trades message"),
        Err(_) => println!("Error")
    }

    let ticker = messages::symbol::SymbolSubscription::new("ticker", messages::symbol::Symbol::new("BTC", "GBP"));
    match tx.send(ticker.to_json()) {
        Ok(_) => println!("Sent trades message"),
        Err(_) => println!("Error")
    }

    let mut message_processor = messages::message_processor::JsonMessageProcessor::new(auth_send, json_recv);
    let order_cache = Arc::new(Mutex::new(Vec::<u64>::with_capacity(64)));
    let message_processing_handler = thread::spawn(move || {
        message_processor.run(order_cache);
    });


    let auth_message = messages::authenticate::Authenticate::new(&api_key);
    match tx.send(auth_message.to_json()) {
        Ok(_) => println!("Sent Auth message"),
        Err(_) => println!("Error")
    }

    // We wait for the auth subscription confirmation before sending messages to authed channels
    auth_recv.recv().unwrap();

    let balances_message = messages::subscribe::Subscribe::new("balances");
    match tx.send(balances_message.to_json()) {
        Ok(_) => println!("Sent balances message"),
        Err(_) => println!("Error")
    }
    let trades_message = messages::subscribe::Subscribe::new("trading");
    match tx.send(trades_message.to_json()) {
        Ok(_) => println!("Sent trading message"),
        Err(_) => println!("Error")
    }

    //place an order -> should be rejected due to no balance
    let order = messages::order::OrderType::Limit {
        symbol: messages::symbol::Symbol::new("BTC", "USD"),
        side: messages::order::OrderSide::SELL,
        price: 12000.0,
        quantity: 1.0,
        time_in_force: messages::order::TimeInForce::GTC
    };
    let new_order_message = messages::order::NewOrderSingle::new(order).unwrap();
    match tx.send(new_order_message.to_json()) {
        Ok(_) => println!("Placed Order: {:?}", &new_order_message),
        Err(_) => println!("Error")
    }

    let _ = message_processing_handler.join().unwrap();
    let thread_result = client_handler.join().unwrap();
    println!("thread result: {:?}", thread_result);
}