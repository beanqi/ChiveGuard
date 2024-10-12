pub enum SignalType {
    Buy,
    Sell,
}

pub enum OrderSide {
    Buy,        // Buy
    Sell,       // Sell
}

pub struct Order {
    pub symbol: String,        // Trading symbol code
    pub amount: f64,         // Order amount, representing the quantity of the buy or sell order (this quantity refers to the number of parts the amount is divided into as defined in the Amount struct)
    pub side: OrderSide,       // Buy or Sell
    pub timestamp: u64,        // Order timestamp
}
