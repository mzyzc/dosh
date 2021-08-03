mod widgets;
mod coin;

use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;

fn main() -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

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

            let graph = widgets::get_graph_block();
            frame.render_widget(graph, bottom[0]);

            let exchange = widgets::get_exchange_block();
            frame.render_widget(exchange, bottom[1]);
        })?;
    }

    Ok(())
}
