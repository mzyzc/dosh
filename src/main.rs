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
use tui::Terminal;
use tui::backend::CrosstermBackend;

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args(env::args());

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let coin_lock = Arc::new(RwLock::new(
        Coin::new(&args.0, &args.1, args.2).expect("Coin data could not be retrieved")
    ));
    let write_lock = Arc::clone(&coin_lock);

    thread::spawn(move || {
        loop {
            if let Ok(mut coin) = write_lock.write() {
                if let Ok(coin_data) = Coin::new(&args.0, &args.1, args.2) {
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

fn parse_args(args: env::Args) -> (String, String, u32) {
    let args: Vec<String> = args.collect();

    let coin = match args.get(1) {
        Some(data) => data.to_owned(),
        None => String::from("ethereum"),
    };

    let currency = match args.get(2) {
        Some(data) => data.to_owned(),
        None => String::from("usd"),
    };

    let days: u32 = match args.get(3) {
        Some(data) => data.parse::<u32>()
            .expect("Invalid 'days' argument"),
        None => 7,
    };
    
    (coin, currency, days)
}