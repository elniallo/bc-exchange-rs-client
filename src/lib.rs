extern crate websocket;
extern crate serde_json;
extern crate serde;

pub mod client;
pub mod messages;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
