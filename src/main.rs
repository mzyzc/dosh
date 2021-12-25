mod coin;
mod price;
mod settings;
mod widgets;

use coin::Coin;
use settings::Settings;

use std::error::Error;
use std::env;
use std::io;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

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

            let price = widgets::get_price_block(&coin);
            frame.render_widget(price, bottom[1]);
        })?;

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}