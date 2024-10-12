use crate::{data::MarketData, signal::Order};
pub mod grid_strategy;

/// A trait that defines a strategy for executing orders based on market data.
/// 
/// # Examples
/// 
/// ```
/// struct MyStrategy;
/// 
/// impl Strategy for MyStrategy {
///     fn execute(&self, data: &MarketData) -> Order {
///         // Implementation of the strategy
///     }
/// }
/// ```
/// 
/// # Methods
/// 
/// * `execute(&self, data: &MarketData) -> Order` - Executes the strategy using the provided market data and returns an order.
pub trait Strategy {
    fn execute(&self, data: &MarketData) -> Option<Order>;
}