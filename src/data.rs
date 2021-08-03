use std::error::Error;
use std::str;
use ureq;

pub fn get_price(coin: &str, currencies: &[&str]) -> Result<String, Box<dyn Error>> {
    let separator = "%2C";
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        coin,
        currencies.join(separator),
    );

    let data = get(&url)?;
    Ok(data)
}

pub fn get_history(coin: &str, currency: &str, days: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days={}",
        coin,
        currency,
        days,
    );

    let data = get(&url)?;
    Ok(data)
}

fn get(url: &str) -> Result<String, Box<dyn Error>> {
    let data = ureq::get(url)
        .call()?
        .into_string()?;
    Ok(data)
}