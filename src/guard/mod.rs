use crate::{data::MarketData, Account};


/// `ChiveGuard` is the main struct that holds the account information and market data.
/// It is used to simulate the trading strategy and execute trading orders.
///
/// # Fields
///
/// * `account` - An instance of the `Account` struct representing the account information.
/// * `data` - A vector containing instances of `MarketData` representing the market data associated with the account.
pub struct ChiveGuard {
    pub account: Account,
    pub data: Vec<MarketData>,
}

impl ChiveGuard {
    pub fn new(data: Vec<MarketData>, account: Account) -> ChiveGuard {
        ChiveGuard { account, data }
    }
}