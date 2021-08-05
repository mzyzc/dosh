use crate::price::Price;

use std::error::Error;
use std::str;
use ureq;

#[derive(Debug)]
pub struct Coin{
    pub name: String,
    pub price: Price,
    pub change: f64,
    pub history: Vec<Price>,
    pub data_points: Vec<(f64, f64)>,
}

impl Coin {
    pub fn new(name: &str, currency: &str, days: u32) -> Result<Coin, Box<dyn Error>> {
        let price = Coin::get_price(name, currency)?;
        let history = Coin::get_history(name, currency, days)?;
        let data_points = Coin::get_data_points(&history);
        let change = Coin::get_change(&data_points);

        Ok(Coin{
            name: String::from(name),
            price: price,
            change: change,
            history: history,
            data_points: data_points,
        })
    }

    pub fn get_price(coin: &str, currency: &str) -> Result<Price, Box<dyn Error>> {
        let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
            coin,
            currency,
        );

        let data = get(&url)?;
        let price = Price::from_price(&data, currency)?;
        Ok(price)
    }

    pub fn get_history(coin: &str, currency: &str, days: u32) -> Result<Vec<Price>, Box<dyn Error>> {
        let url = format!("https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days={}",
            coin,
            currency,
            days,
        );

        let data = get(&url)?;
        let history = Price::from_history(&data, currency)?;
        Ok(history)
    }

    fn get_data_points(history: &[Price]) -> Vec<(f64, f64)> {
        history
            .iter()
            .map(|c| (c.timestamp.timestamp() as f64, c.value as f64))
            .collect()
    }

    fn get_change(prices: &[(f64, f64)]) -> f64 {
        let oldest = match prices.first() {
            Some(data) => data.1,
            None => 0.0,
        };

        let newest = match prices.last() {
            Some(data) => data.1,
            None => 0.0,
        };
        
        ((newest - oldest) / oldest) * 100.0
    }
}

fn get(url: &str) -> Result<String, Box<dyn Error>> {
    let data = ureq::get(url)
        .call()?
        .into_string()?;
    Ok(data)
}