# Wex API

Full API implementation for the [Wex](https://wex.nz/) market-place.

**Please Donate**

+ **BTC:** 17voJDvueb7iZtcLRrLtq3dfQYBaSi2GsU
+ **ETC:** 0x7bC5Ff6Bc22B4C6Af135493E6a8a11A62D209ae5
+ **XMR:** 49S4VziJ9v2CSkH6mP9km5SGeo3uxhG41bVYDQdwXQZzRF6mG7B4Fqv2aNEYHmQmPfJcYEnwNK1cAGLHMMmKaUWg25rHnkm

**Wex API Documentation:**
+ https://wex.nz/api/3/docs
+ https://wex.nz/tapi/docs


## Example

```rust
extern crate wex;

fn main() {
   let account = wex::Account {
        key: String::from("<your-key>"),
        secret: String::from("<your-secret>"),
    };

    println!("{:?}", wex::info());
    println!("{:?}", wex::get_info(&account));

    // currency pair chain :)

    let info = wex::info().expect("could not optain wex pairs");
    let pairs: Vec<&String> = info.pairs.keys().collect();
    let mut pairchain = pairs.iter().fold(
        String::new(),
        |data, item| data + item + "-",
    );
    pairchain.pop(); // remove last `-`

    // ticker all pairs at once :D
    let ticks = wex::ticker(&pairchain).expect("could not ticker");
}
```
