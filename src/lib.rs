//!
//! # Wex API
//!
//! API implementation for the [Wex](https://wex.nz/) market-place.
//!
//! **Please Donate**
//!
//! + **BTC:** 17voJDvueb7iZtcLRrLtq3dfQYBaSi2GsU
//! + **ETC:** 0x7bC5Ff6Bc22B4C6Af135493E6a8a11A62D209ae5
//! + **XMR:** 49S4VziJ9v2CSkH6mP9km5SGeo3uxhG41bVYDQdwXQZzRF6mG7B4Fqv2aNEYHmQmPfJcYEnwNK1cAGLHMMmKaUWg25rHnkm
//!
//! **Wex API Documentation:**
//! + https://wex.nz/api/3/docs
//! + https://wex.nz/tapi/docs
//!
extern crate crypto;
extern crate curl;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::Read;
use curl::easy::{Easy, List};


///
/// Representing a key secret pair from Wex.
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub key: String,
    pub secret: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TickPair {
    pub high: f64,
    pub low: f64,
    pub avg: f64,
    pub vol: f64,
    pub vol_cur: f64,
    pub last: f64,
    pub buy: f64,
    pub sell: f64,
    pub updated: i64,
}

pub type Tick = HashMap<String, TickPair>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InfoPair {
    pub decimal_places: u32,
    pub min_price: f64,
    pub max_price: f64,
    pub min_amount: f64,
    pub hidden: u32,
    pub fee: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Info {
    pub server_time: i64,
    pub pairs: HashMap<String, InfoPair>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WexResult<T> {
    success: u8,
    #[serde(rename = "return")]
    result: Option<T>,
    error: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FundRights {
    pub info: u8,
    pub trade: u8,
    pub withdraw: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FundInfo {
    /// Your account balance available for trading. Doesn’t include funds on your open orders.
    pub funds: HashMap<String, f64>,
    /// The privileges of the current API key. At this time the privilege to withdraw is not used anywhere.
    pub rights: FundRights,
    /// Deprecated, is equal to 0.
    pub transaction_count: u32,
    /// The number of your open orders.
    pub open_orders: u32,
    /// Server time (MSK).
    pub server_time: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradeResult {
    /// The amount of currency bought/sold.
    received: f64,
    /// The remaining amount of currency to be bought/sold (and the initial order amount).
    remains: f64,
    /// Is equal to 0 if the request was fully “matched” by the opposite orders, otherwise the ID of the executed order will be returned.
    order_id: u64,
    /// Balance after the request.
    funds: HashMap<String, f64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Sell,
    Buy,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum OrderStatus {
    Active = 0,
    ExecutedOrder = 1,
    Canceled = 2,
    PartiallyExecuted = 3,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ActiveOrder {
    /// The pair on which the order was created.
    pair: String,
    ///  Order type, buy/sell.
    #[serde(rename = "type")]
    kind: OrderType,
    /// The amount of currency to be bought/sold.
    amount: f64,
    /// Sell/Buy price.
    rate: f64,
    /// The time when the order was created.
    timestamp_created: i64,
    /// Deprecated, is always equal to 0.
    status: OrderStatus,
}

pub type ActiveOrders = HashMap<String, ActiveOrder>;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrderInfo {
    /// The pair on which the order was created.
    pair: String,
    ///  Order type, buy/sell.
    #[serde(rename = "type")]
    kind: OrderType,
    /// The initial amount at the time of order creation.
    start_amount: f64,
    /// The remaining amount of currency to be bought/sold.
    amount: f64,
    /// Sell/Buy price.
    rate: f64,
    /// The time when the order was created.
    timestamp_created: i64,
    /// Deprecated, is always equal to 0.
    status: OrderStatus,
}

pub type OrderInfos = HashMap<String, ActiveOrder>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CancelResult {
    /// The ID of canceled order.
    order_id: String,
    /// Balance upon request.
    funds: HashMap<String, f64>,
}


pub enum TradeHistorySorting {
    Asc,
    Desc,
}

pub struct HistoryQuery {
    /// trade ID, from which the display starts 	numerical 	0
    from: Option<u64>,
    /// the number of trades for display 	numerical 	1000
    count: Option<u64>,
    /// trade ID, from which the display starts 	numerical 	0
    from_id: Option<u64>,
    /// trade ID on which the display ends 	numerical 	∞
    end_id: Option<u64>,
    /// Sorting. Defaults: DESC.
    order: Option<TradeHistorySorting>,
    /// the time to start the display 	UNIX time 	0
    since: Option<i64>,
    /// the time to end the display 	UNIX time 	∞
    end: Option<i64>,
    /// pair to be displayed 	btc_usd (example) 	all pairs
    pair: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradeHistoryEntry {
    /// The pair on which the trade was executed.
    pair: String,
    /// Trade type, buy/sell.
    kind: OrderType,
    /// The amount of currency was bought/sold.
    amount: f64,
    /// Sell/Buy price.
    rate: f64,
    /// Order ID.
    order_id: u64,
    /// Is equal to 1 if order_id is your order, otherwise is equal to 0.
    is_your_order: u8,
    /// Trade execution time.
    timestamp: i64,
}

type TradeHistory = HashMap<String, TradeHistoryEntry>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TransactionType {
    Deposit = 1,
    Withdrawl = 2,
    Credit = 4,
    Debit = 5,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TransactionStatus {
    Canceled = 0,
    Waiting = 1,
    Successful = 2,
    NotConfirmed = 3,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransactionHistoryEntry {
    #[serde(rename = "type")]
    kind: TransactionType,
    amount: f64,
    currency: String,
    desc: String,
    status: TransactionStatus,
    timestamp: i64,
}

pub type TransactionHistory = HashMap<String, TransactionHistoryEntry>;

fn public(url: &str) -> Result<Vec<u8>, String> {
    let mut easy = Easy::new();
    let mut dst = Vec::new();

    easy.url(&format!("https://wex.nz/api/3/{}", url)).unwrap();

    let result = {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform()
    };

    result.map_err(|e| format!("{:?}", e)).and_then(|x| Ok(dst))
}

///
/// This method provides all the information about currently active pairs,
/// such as the maximum number of digits after the decimal point,
/// the minimum price, the maximum price, the minimum transaction size,
/// whether the pair is hidden, the commission for each pair.
///
pub fn info() -> Result<Info, String> {
    public("info").and_then(|data| {
        serde_json::from_slice(&data).map_err(|e| format!("{:?}", e))
    })
}


pub fn ticker(pair: &str) -> Result<Tick, String> {
    public(&format!("ticker/{}", pair)).and_then(|data| {
        serde_json::from_slice(&data).map_err(|e| format!("{:?}", e))
    })
}

// // ---

fn private(account: &Account, params: &mut HashMap<String, String>) -> Result<Vec<u8>, String> {
    let mut dst = Vec::new();
    let mut easy = Easy::new();

    easy.url("https://wex.nz/tapi").unwrap();
    easy.post(true).unwrap();

    let timestamp = ::std::time::UNIX_EPOCH.elapsed().unwrap();
    let nonce = format!("{}", timestamp.as_secs());

    params.insert("nonce".to_owned(), nonce);

    let mut body = params.iter().fold(
        String::new(),
        |data, item| data + item.0 + "=" + item.1 + "&",
    );
    body.pop();
    let mut body_bytes = body.as_bytes();

    let mut hmac = Hmac::new(Sha512::new(), account.secret.as_bytes());

    hmac.input(body_bytes);

    easy.post_field_size(body_bytes.len() as u64).unwrap();

    let mut list = List::new();
    let sign = hmac.result();

    let mut hex = String::new();
    for byte in sign.code() {
        write!(&mut hex, "{:02x}", byte).expect("could not create hmac hex");
    }
    list.append("Content-Type: application/x-www-form-urlencoded")
        .unwrap();
    list.append(&format!("Key: {}", account.key)).unwrap();
    list.append(&format!("Sign: {}", hex)).unwrap();

    easy.http_headers(list).unwrap();

    let result = {
        let mut transfer = easy.transfer();

        transfer
            .read_function(|buf| Ok(body_bytes.read(buf).unwrap_or(0)))
            .unwrap();

        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap()
    };

    Ok(dst)
}

///
/// Returns information about the user’s current balance,
/// API-key privileges, the number of open orders and Server Time.
/// To use this method you need a privilege of the key info.
///
pub fn get_info(account: &Account) -> Result<FundInfo, String> {
    let mut params = HashMap::new();

    params.insert("method".to_owned(), "getInfo".to_owned());

    private(account, &mut params).and_then(|r| {
        serde_json::from_slice(&r)
            .map_err(|e| format!("{:?}", e))
            .and_then(|result: WexResult<FundInfo>| if result.success == 1 {
                Ok(result.result.unwrap())
            } else {
                Err(result.error.unwrap())
            })
    })
}

///
/// You can only create limit orders using this method, but you can emulate market orders using rate parameters.
/// E.g. using rate=0.1 you can sell at the best market price.
/// Each pair has a different limit on the minimum / maximum amounts, the minimum amount and the number of digits after the decimal point.
/// All limitations can be obtained using the info method in PublicAPI v3.
///
pub fn trade(
    account: &Account,
    pair: &str,
    kind: OrderType,
    rate: &str,
    amount: &str,
) -> Result<TradeResult, String> {
    let mut params = HashMap::new();

    let kind_str = match kind {
        OrderType::Buy => String::from("buy"),
        OrderType::Sell => String::from("sell"),
    };

    params.insert("method".to_owned(), "Trade".to_owned());
    params.insert("pair".to_owned(), String::from(pair));
    params.insert("type".to_owned(), kind_str);
    params.insert("rate".to_owned(), String::from(rate));
    params.insert("amount".to_owned(), String::from(amount));

    private(account, &mut params).and_then(|r| {
        serde_json::from_slice(&r)
            .map_err(|e| format!("{:?}", e))
            .and_then(|result: WexResult<TradeResult>| if result.success == 1 {
                Ok(result.result.unwrap())
            } else {
                Err(result.error.unwrap())
            })
    })
}

///
/// Returns the list of your active orders.
/// If the order disappears from the list, it was either executed or canceled.
///
pub fn active_orders(account: &Account, pair: Option<&str>) -> Result<ActiveOrders, String> {
    let mut params = HashMap::new();

    params.insert("method".to_owned(), "ActiveOrders".to_owned());

    if let Some(p) = pair {
        params.insert("pair".to_owned(), String::from(p));
    }

    private(account, &mut params).and_then(|r| {
        serde_json::from_slice(&r)
            .map_err(|e| format!("{:?}", e))
            .and_then(|result: WexResult<ActiveOrders>| if result.success == 1 {
                Ok(result.result.unwrap())
            } else {
                Err(result.error.unwrap())
            })
    })
}

///
/// Returns the information on particular order.
///
pub fn order_info(account: &Account, order_id: &str) -> Result<OrderInfos, String> {
    let mut params = HashMap::new();

    params.insert("method".to_owned(), "OrderInfo".to_owned());
    params.insert("order_id".to_owned(), String::from(order_id));

    private(account, &mut params).and_then(|r| {
        serde_json::from_slice(&r)
            .map_err(|e| format!("{:?}", e))
            .and_then(|result: WexResult<OrderInfos>| if result.success == 1 {
                Ok(result.result.unwrap())
            } else {
                Err(result.error.unwrap())
            })
    })
}

///
/// This method is used for order cancelation.
///
pub fn cancel_order(account: &Account, order_id: &str) -> Result<CancelResult, String> {
    let mut params = HashMap::new();

    params.insert("method".to_owned(), "CancelOrder".to_owned());
    params.insert("order_id".to_owned(), String::from(order_id));

    private(account, &mut params).and_then(|r| {
        serde_json::from_slice(&r)
            .map_err(|e| format!("{:?}", e))
            .and_then(|result: WexResult<CancelResult>| if result.success == 1 {
                Ok(result.result.unwrap())
            } else {
                Err(result.error.unwrap())
            })
    })
}

///
/// Returns trade history.
///
/// When using parameters since or end, the order parameter automatically assumes the value ASC.
/// When using the since parameter the maximum time that can displayed is 1 week.
///
pub fn trade_history(account: &Account, cfg: Option<HistoryQuery>) -> Result<TradeHistory, String> {
    let mut params = HashMap::new();

    params.insert("method".to_owned(), "TradeHistory".to_owned());

    if let Some(p) = cfg {
        if let Some(from) = p.from {
            params.insert("from".to_owned(), format!("{}", from));
        }

        if let Some(count) = p.count {
            params.insert("count".to_owned(), format!("{}", count));
        }

        if let Some(from_id) = p.from_id {
            params.insert("from_id".to_owned(), format!("{}", from_id));
        }

        if let Some(end_id) = p.end_id {
            params.insert("end_id".to_owned(), format!("{}", end_id));
        }

        if let Some(order) = p.order {
            match order {
                TradeHistorySorting::Asc => params.insert("order".to_owned(), "asc".to_owned()),
                TradeHistorySorting::Desc => params.insert("order".to_owned(), "desc".to_owned()),
            };
        }

        if let Some(since) = p.since {
            params.insert("since".to_owned(), format!("{}", since));
        }

        if let Some(end) = p.end {
            params.insert("end".to_owned(), format!("{}", end));
        }

        if let Some(pair) = p.pair {
            params.insert("pair".to_owned(), pair);
        }
    }

    private(account, &mut params).and_then(|r| {
        serde_json::from_slice(&r)
            .map_err(|e| format!("{:?}", e))
            .and_then(|result: WexResult<TradeHistory>| if result.success == 1 {
                Ok(result.result.unwrap())
            } else {
                Err(result.error.unwrap())
            })
    })
}

pub fn trans_history(
    account: &Account,
    cfg: Option<HistoryQuery>,
) -> Result<TransactionHistory, String> {
    let mut params = HashMap::new();

    params.insert("method".to_owned(), "TransHistory".to_owned());

    if let Some(p) = cfg {
        if let Some(from) = p.from {
            params.insert("from".to_owned(), format!("{}", from));
        }

        if let Some(count) = p.count {
            params.insert("count".to_owned(), format!("{}", count));
        }

        if let Some(from_id) = p.from_id {
            params.insert("from_id".to_owned(), format!("{}", from_id));
        }

        if let Some(end_id) = p.end_id {
            params.insert("end_id".to_owned(), format!("{}", end_id));
        }

        if let Some(order) = p.order {
            match order {
                TradeHistorySorting::Asc => params.insert("order".to_owned(), "asc".to_owned()),
                TradeHistorySorting::Desc => params.insert("order".to_owned(), "desc".to_owned()),
            };
        }

        if let Some(since) = p.since {
            params.insert("since".to_owned(), format!("{}", since));
        }

        if let Some(end) = p.end {
            params.insert("end".to_owned(), format!("{}", end));
        }

        if let Some(pair) = p.pair {
            params.insert("pair".to_owned(), pair);
        }
    }

    private(account, &mut params).and_then(|r| {
        serde_json::from_slice(&r)
            .map_err(|e| format!("{:?}", e))
            .and_then(|result: WexResult<TransactionHistory>| if result.success ==
                1
            {
                Ok(result.result.unwrap())
            } else {
                Err(result.error.unwrap())
            })
    })
}

///
/// This method can be used to retrieve the address for depositing crypto-currency.
///
/// To use this method, you need the info key privilege.
///
/// At present, this method does not generate new adresses.
/// If you have never deposited in a particular crypto-currency and try to retrive a deposit address,
/// your request will return an error, because this address has not been generated yet.
///
pub fn coin_deposit_address(account: &Account, coin_name: &str) -> Result<CancelResult, String> {
    let mut params = HashMap::new();

    params.insert("method".to_owned(), "CoinDepositAddress".to_owned());
    params.insert("coinName".to_owned(), String::from(coin_name));

    private(account, &mut params).and_then(|r| {
        serde_json::from_slice(&r)
            .map_err(|e| format!("{:?}", e))
            .and_then(|result: WexResult<CancelResult>| if result.success == 1 {
                Ok(result.result.unwrap())
            } else {
                Err(result.error.unwrap())
            })
    })
}
