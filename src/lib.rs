pub mod data;
pub mod signal;
pub mod guard;
pub mod strategy;

/// A struct representing a financial position.
///
/// # Fields
///
/// * `symbol` - The code of the trading instrument.
/// * `quantity` - The quantity of the position held.
/// * `average_cost` - The average cost price of the position.
/// * `unrealized_pnl` - The unrealized profit and loss (floating P&L) of the position.
pub struct Position {
    pub symbol: String,        
    pub quantity: f64,        
    pub average_cost: f64,     
    pub unrealized_pnl: f64,  
}
/// Represents a user's account in the system.
///
/// The `Account` struct holds information about the user's financial status,
/// including their balance, positions, and orders.
///
/// # Fields
///
/// * `balance` - The current balance of the account.
/// * `partition_count` - The number of partitions in the account, representing how many parts the total balance is divided into.
/// * `positions` - A list of positions held in the account.
/// * `orders` - A list of orders associated with the account.
pub struct Account {
    pub balance: f64,
    pub partition_count: u32,
    pub positions: Vec<Position>,
    pub orders: Vec<signal::Order>,
}



impl Account {

    /// Creates a new `Account` instance with the specified balance and partition count.
    ///
    /// # Arguments
    ///
    /// * `balance` - A floating-point number representing the initial balance of the account.
    /// * `partition_count` - An unsigned integer representing the number of partitions for the account.
    ///
    /// # Returns
    ///
    /// * `Account` - A new instance of the `Account` struct with the specified balance and partition count,
    ///   and empty vectors for positions and orders.
    ///
    /// # Examples
    ///
    /// ```
    /// let account = Account::new(1000.0, 5);
    /// assert_eq!(account.balance, 1000.0);
    /// assert_eq!(account.partition_count, 5);
    /// assert!(account.positions.is_empty());
    /// assert!(account.orders.is_empty());
    /// ```
    pub fn new(balance: f64, partition_count: u32) -> Account {
        let (positions, orders) = (Vec::new(), Vec::new());
        Account {
            balance,
            partition_count,
            positions,
            orders,
        }
    }
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
