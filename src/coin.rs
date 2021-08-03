use std::error::Error;
use std::str;
use ureq;

pub struct Coin{
    name: String,
}

impl Coin {
    pub fn get_price(&self, currencies: &[&str]) -> Result<String, Box<dyn Error>> {
        let separator = "%2C";
        let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24h_change=true",
            &self.name,
            currencies.join(separator),
        );

        let data = get(&url)?;
        Ok(data)
    }

    pub fn get_history(&self, currency: &str, days: u32) -> Result<String, Box<dyn Error>> {
        let url = format!("https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days={}",
            &self.name,
            currency,
            days,
        );

        let data = get(&url)?;
        Ok(data)
    }
}

fn get(url: &str) -> Result<String, Box<dyn Error>> {
    let data = ureq::get(url)
        .call()?
        .into_string()?;
    Ok(data)
}