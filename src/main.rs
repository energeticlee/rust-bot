use reqwest::Client;
use std::time::Duration;
use tokio::sync::mpsc;

mod helper_fuctions;
mod market_data;
mod modules;

use helper_fuctions::fetch_data;
use market_data::MarketData;
use modules::trading_bot::TradingBot;
use modules::trading_platforms::binance::Binance;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let binance = Binance {};
    let bot = TradingBot::new("binance_bot", binance);

    let (sender, receiver) = mpsc::channel::<MarketData>(100);

    tokio::spawn(async move {
        loop {
            let response = fetch_data(&client).await.expect("Failed to fetch data");

            let data = response
                .json::<MarketData>()
                .await
                .expect("Failed to parse JSON");
            sender.send(data).await.expect("Failed to send data");

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    bot.start(receiver).await;
}
