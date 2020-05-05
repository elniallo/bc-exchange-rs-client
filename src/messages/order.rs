use serde::{Serialize, Deserialize};
use rand::Rng;
use crate::messages::symbol::Symbol;

#[derive(Deserialize, Serialize, Debug)]
pub struct NewOrderSingle {
    action: String,
    channel: String,
    clOrdId: String,
    symbol: String,
    ordType: String,
    timeInForce: String,
    side: String,
    orderQty: f64,
    price: f64,
    execInst: String
}

impl NewOrderSingle {
    pub fn new(order: OrderType) -> Result<Self,()> {
        let mut rng = rand::thread_rng();
        let order_id: u64 = rng.gen();
        match order {
            OrderType::Limit {symbol, time_in_force, side, quantity, amount} => {
                Ok(NewOrderSingle {
                    action: String::from("NewOrderSingle"),
                    channel: String::from("trading"),
                    clOrdId: order_id.to_string(),
                    ordType: "LIMIT".to_string(),
                    symbol: symbol.as_pair(),
                    timeInForce: time_in_force.to_string(),
                    side: side.to_string(),
                    orderQty: quantity,
                    price: amount,
                    execInst: "ALO".to_string()
                })
            }
            _ => Err(())
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderCancel {
    order_id: u64,
    channel: String,
    action: String
}

impl OrderCancel {
    pub fn new(order_id: &u64) -> Self {
        OrderCancel {
            action: String::from("subscribe"),
            channel: String::from("auth"),
            order_id: *order_id
        }
    }
}

pub enum OrderType<'a> {
    Limit { symbol: Symbol<'a>, time_in_force: TimeInForce, side: OrderSide, quantity: f64, amount: f64 },
    Market { symbol:Symbol<'a>, side: OrderSide, quantity: f64},
    Stop { symbol: Symbol<'a>, side: OrderSide, stop_price: f64},
    StopLimit { symbol: Symbol<'a>, side: OrderSide, price: f64, stop_price: f64 }
}

pub enum OrderSide {
    BUY,
    SELL
}

impl ToString for OrderSide {
    fn to_string(&self) -> String {
        match self {
            BUY => String::from("BUY"),
            SELL => String::from("SELL")
        }
    }
}

pub enum TimeInForce {
    GTC,
    GTD { expiry: u64 },
    FOK,
    IOC
}

impl ToString for TimeInForce {
    fn to_string(&self) -> String {
        let mut out = String::new();
        match self {
            GTC => out.push_str("GTC"),
            GTD => out.push_str("GTD"),
            FOK => out.push_str("FOK"),
            IOC => out.push_str("IOC")
        };
        out
    }
}