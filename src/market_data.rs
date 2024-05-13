// Define a struct to hold market data
#[derive(Debug, Clone, serde::Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
}
