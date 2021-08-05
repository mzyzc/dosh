mod coin;
mod price;
mod widgets;

use coin::Coin;

use std::error::Error;
use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;

fn main() -> Result<(), Box<dyn Error>> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let coin = Coin::new("ethereum", "gbp", 7)?;
    println!("{:#?}", &coin.data_points);

    loop {
        terminal.draw(|frame| {
            let chunks = widgets::get_chunks()
                .split(frame.size());

            let top = widgets::get_top_chunk()
                .split(chunks[0]);

            let bottom = widgets::get_bottom_chunk()
                .split(chunks[1]);

            let timescale = widgets::get_timescale_tabs();
            frame.render_widget(timescale, top[0]);

            let change = widgets::get_change_block();
            frame.render_widget(change, top[1]);

            let graph = widgets::get_graph(&coin);
            frame.render_widget(graph, bottom[0]);

            let exchange = widgets::get_exchange_block();
            frame.render_widget(exchange, bottom[1]);
        })?;
    }

    Ok(())
}
