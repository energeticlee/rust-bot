use crate::market_data::MarketData;
use rand::Rng;
use std::error::Error;
use std::sync::{Arc, Mutex, TryLockError};
use tokio::sync::mpsc;

pub trait PlatformRequirements {
    fn place_order(
        &self,
        symbol: &str,
        action: &str,
        quantity: f64,
        price: Option<f64>,
    ) -> Result<(), Box<dyn Error>>;
    fn check_balance(&self, currency: &str) -> Result<f64, Box<dyn Error>>;
    // Add other methods as needed
}

pub struct TradingBot<T: PlatformRequirements> {
    name: String,
    platform: Arc<Mutex<T>>,
}

impl<T: PlatformRequirements> TradingBot<T> {
    pub fn new(name: &str, platform: T) -> Self {
        TradingBot {
            name: name.to_string(),
            platform: Arc::new(Mutex::new(platform)),
        }
    }

    pub async fn start(&self, mut receiver: mpsc::Receiver<MarketData>) {
        while let Some(data) = receiver.recv().await {
            println!("{:?} received market data: {:?}", self.name, data);

            let decision = rand::thread_rng().gen_range(0..2); // 0 or 1
            let action = if decision == 0 { "BUY" } else { "SELL" };

            self.place_order(&data.symbol, action, 1.0, Some(data.price))
                .unwrap();
        }
    }

    pub fn place_order(
        &self,
        symbol: &str,
        action: &str,
        quantity: f64,
        price: Option<f64>,
    ) -> Result<(), Box<dyn Error>> {
        let platform_lock = match self.platform.try_lock() {
            Ok(lock) => lock,
            Err(TryLockError::Poisoned(_)) => {
                return Err("Lock poisoned".into());
            }
            Err(TryLockError::WouldBlock) => {
                return Err("Failed to acquire lock".into());
            }
        };

        platform_lock.place_order(symbol, action, quantity, price)?;

        Ok(())
    }
}
