pub mod market_data;

pub struct MarketData {
    pub symbol: String,        // Trading symbol, e.g., "BTC/USD"
    pub timestamp: u64,        // Data timestamp, usually in milliseconds
    pub open: f64,             // Opening price
    pub high: f64,             // Highest price
    pub low: f64,              // Lowest price
    pub close: f64,            // Closing price
    pub volume: f64,           // Trading volume
}

impl MarketData {
    pub fn new(symbol: &str, timestamp: u64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> MarketData {
        MarketData {
            symbol: symbol.to_string(),
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}