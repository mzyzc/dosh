use crate::price::Price;

use std::error::Error;
use std::str;
use ureq;

pub struct Coin{
    name: String,
    price: String,
    history: String,
}

impl Coin {
    pub fn new(name: &str, currency: &str, days: u32) -> Result<Coin, Box<dyn Error>> {
        let price = Coin::get_price(name, currency)?;
        let history = Coin::get_history(name, currency, days)?;

        Ok(Coin{
            name: String::from(name),
            price: price,
            history: history,
        })
    }

    pub fn get_price(coin: &str, currency: &str) -> Result<String, Box<dyn Error>> {
        let separator = "%2C";
        let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24h_change=true",
            coin,
            currency,
        );

        let data = get(&url)?;
        println!("{:#?}\n",
            Price::from_price(&data, &currency)?
        );
        Ok(data)
    }

    pub fn get_history(coin: &str, currency: &str, days: u32) -> Result<String, Box<dyn Error>> {
        let url = format!("https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days={}",
            coin,
            currency,
            days,
        );

        let data = get(&url)?;
        println!("{:#?}\n",
            Price::from_history(&data, currency)?
        );
        Ok(data)
    }
}

fn get(url: &str) -> Result<String, Box<dyn Error>> {
    let data = ureq::get(url)
        .call()?
        .into_string()?;
    Ok(data)
}