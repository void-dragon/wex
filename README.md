# Wex API

API implementation for the [Wex](https://wex.nz/) market-place.

**Please Donate**

+ **ETC:** 0x7bC5Ff6Bc22B4C6Af135493E6a8a11A62D209ae5
+ **XMR:** 49S4VziJ9v2CSkH6mP9km5SGeo3uxhG41bVYDQdwXQZzRF6mG7B4Fqv2aNEYHmQmPfJcYEnwNK1cAGLHMMmKaUWg25rHnkm

**Wex API Documentation:**
+ https://wex.nz/api/3/docs
+ https://wex.nz/tapi/docs


## Example

```rust
extern crate wex;

fn main() {
  let mut api = wex::Wex::new();
  
  api
    .ticker("btc_usd,eth_usd")
    .map_err(|e| println!("a buh-buh happend, {}", e))
    .and_then(|tick| {
      if let Some(info) = tick.get("btc_usd") {
        println!("{:?}", info.sell);
      }
    });
}
```
