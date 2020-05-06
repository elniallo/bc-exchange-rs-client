use serde::{Serialize, Deserialize};
use rand::Rng;
use chrono::{DateTime, TimeZone, Utc};
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
    execInst: String,
    expireDate: String
}

impl NewOrderSingle {
    pub fn new(order: OrderType) -> Result<Self,()> {
        let mut rng = rand::thread_rng();
        let order_id: u64 = rng.gen();
        match order {
            OrderType::Limit {symbol, time_in_force, side, quantity, price} => {
                let expire_date = match time_in_force {
                    TimeInForce::GTD { expiry } => {expiry.naive_utc().date().to_string()},
                    _ => String::default()
                };
                Ok(NewOrderSingle {
                    action: String::from("NewOrderSingle"),
                    channel: String::from("trading"),
                    clOrdId: order_id.to_string(),
                    ordType: "limit".to_string(),
                    symbol: symbol.as_pair(),
                    timeInForce: time_in_force.to_string(),
                    side: side.to_string(),
                    orderQty: quantity,
                    price: price,
                    execInst: "ALO".to_string(),
                    expireDate: expire_date
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
    orderID: u64,
    channel: String,
    action: String
}

impl OrderCancel {
    pub fn new(order_id: &u64) -> Self {
        OrderCancel {
            action: String::from("subscribe"),
            channel: String::from("auth"),
            orderID: *order_id
        }
    }
}

pub enum OrderType<'a> {
    Limit { symbol: Symbol<'a>, time_in_force: TimeInForce, side: OrderSide, quantity: f64, price: f64 },
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
    GTD { expiry: DateTime<Utc> },
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn correctly_creates_gtd_order() {
        let date: DateTime<Utc> = DateTime::from_str("2020-09-26T18:00:00z").unwrap();
        let order = OrderType::Limit {
            symbol: Symbol::new("ETH", "GBP"),
            time_in_force: TimeInForce::GTD {
                expiry: date
            },
            side: OrderSide::BUY,
            quantity: 1.0,
            price: 1.0
        };
        let newOrder = NewOrderSingle::new(order).unwrap();
        assert_eq!(newOrder.expireDate, "2020-09-26");
    }
}