# bc-exchange-rs-client
Rust Implementation of Blockchain Exchange Websocket Api

## Example
If you wish to test authenticated routes, paste your Blockchain exchange API key where it says "YOUR API KEY HERE" in the example.

To run the e2e example - simply type.
```
cargo run --example e2e
```

# TODO
1. Deserialisation of responses into useable structs
2. Split socket into reader/writer and move to separate threads
3. Possibly move to futures based implementation
