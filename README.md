# Wex API

Full API implementation for the [Wex](https://wex.nz/) market-place.

**Wex API Documentation:**
+ https://wex.nz/api/3/docs
+ https://wex.nz/tapi/docs

**Documentation:**  https://docs.rs/wex/ ![](https://docs.rs/wex/badge.svg)

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

    // ticker all pairs at once
    let ticks = wex::ticker(&pairchain).expect("could not ticker");
}
```
