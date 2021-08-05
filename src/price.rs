use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use json;

#[derive(Debug)]
pub struct Price {
    pub value: f32,
    pub currency: String,
    pub timestamp: Duration,
}

impl Price {
    pub fn from_price(json: &str, currency: &str) -> Result<Price, Box<dyn Error>> {
        let data = json::parse(json)?;
        let (_coin, data) = data
            .entries()
            .next()
            .ok_or_else(|| "Could not parse JSON")?;

        let value = data[currency]
            .as_f32()
            .ok_or_else(|| "Currency value does not seem to be a number")?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0));

        Ok(
            Price{
                value,
                currency: String::from(currency),
                timestamp,
            }
        )
    }

    pub fn from_history(json: &str, currency: &str) -> Result<Vec<Price>, Box<dyn Error>> {
        let data = &json::parse(json)?;

        let prices: Vec<Price> = data["prices"]
            .members()
            .map(|values| {
                let time = values[0].as_i64().unwrap() as u64;
                let value = values[1].as_f32().unwrap();
                Price{
                    value: value,
                    currency: String::from(currency),
                    timestamp: Duration::from_millis(time),
                }
            })
            .collect();

        Ok(prices)
    }
}