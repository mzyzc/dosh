use std::env;
use rust_decimal::prelude::*;

pub struct Settings {
    pub coin: String,
    pub quantity: Decimal,
    pub days: u32,
    pub currency: String,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings{
            coin: String::from("bitcoin"),
            quantity: Decimal::new(1, 0),
            days: 7,
            currency: String::from("usd"),
        }
    }
}

impl Settings {
    pub fn parse(args: env::Args) -> Settings {
        let args: Vec<String> = args.collect();

        let mut output = Settings::default();

        for arg in args[1..].iter() {
            let mut split = arg.split('=');
            let (key, value) = (
                split.next().expect("Invalid option formatting"),
                split.next().expect("Invalid option formatting"),
            );

            match key {
                "coin" => output.coin = String::from(value),
                "quantity" => output.quantity = value.parse().expect("Invalid 'quantity' argument"),
                "days" => output.days = value.parse().expect("Invalid 'days' argument"),
                "currency" => output.currency = String::from(value),
                _ => {},
            }
        }

        output
    }
}