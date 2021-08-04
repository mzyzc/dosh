use std::error::Error;
use chrono::prelude::*;
use json;

#[derive(Debug)]
pub struct Price {
    value: f32,
    currency: String,
    timestamp: DateTime<Utc>,
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

        Ok(
            Price{
                value: value,
                currency: String::from(currency),
                timestamp: Utc::now(),
            }
        )
    }

    pub fn from_history(json: &str, currency: &str) -> Result<Vec<Price>, Box<dyn Error>> {
        let data = &json::parse(json)?;

        let prices: Vec<Price> = data["prices"]
            .members()
            .map(|values| {
                let epoch = values[0].as_i64().unwrap();
                let value = values[1].as_f32().unwrap();
                Price{
                    value: value,
                    currency: String::from(currency),
                    timestamp: Utc.timestamp(epoch, 0),
                }
            })
            .collect();

        Ok(prices)
    }
}