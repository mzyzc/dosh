mod coin;
mod price;
mod widgets;

use coin::Coin;

use std::error::Error;
use std::env;
use std::io;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use rust_decimal::prelude::*;
use tui::Terminal;
use tui::backend::CrosstermBackend;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Settings::parse(env::args());

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let coin_lock = Arc::new(RwLock::new(
        Coin::new(&opts.coin, opts.quantity, opts.days, &opts.currency).expect("Coin data could not be retrieved")
    ));
    let write_lock = Arc::clone(&coin_lock);

    thread::spawn(move || {
        loop {
            if let Ok(mut coin) = write_lock.write() {
                if let Ok(coin_data) = Coin::new(&opts.coin, opts.quantity, opts.days, &opts.currency) {
                    *coin = coin_data;
                }
            }
            thread::sleep(Duration::from_secs(60));
        }
    });

    let mut coin = coin_lock.read()
        .expect("Coin data could not be read")
        .clone();

    loop {
        if let Ok(c) = coin_lock.try_read() {
            coin = (*c).clone();
        }

        terminal.draw(|frame| {
            let chunks = widgets::get_chunks()
                .split(frame.size());

            let top = widgets::get_top_chunk()
                .split(chunks[0]);

            let bottom = widgets::get_bottom_chunk()
                .split(chunks[1]);

            let timescale = widgets::get_timescale_tabs();
            frame.render_widget(timescale, top[0]);

            let change = widgets::get_change_block(&coin);
            frame.render_widget(change, top[1]);

            let graph = widgets::get_graph(&coin);
            frame.render_widget(graph, bottom[0]);

            let exchange = widgets::get_exchange_block(&coin);
            frame.render_widget(exchange, bottom[1]);
        })?;

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

struct Settings {
    pub coin: String,
    pub quantity: Decimal,
    pub days: u32,
    pub currency: String,
}

impl Settings {
    pub fn parse(args: env::Args) -> Settings {
        let args: Vec<String> = args.collect();

        let mut output = Settings{
            coin: String::from("ethereum"),
            quantity: Decimal::new(1, 0),
            days: 7,
            currency: String::from("usd"),
        };

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