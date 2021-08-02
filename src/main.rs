use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(90),
                ].as_ref()
            )
            .split(frame.size());

        let top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ].as_ref()
            )
            .split(chunks[0]);

        let bottom = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ].as_ref()
            )
            .split(chunks[1]);

        let timescale_block = Block::default()
            .title("Timescale")
            .borders(Borders::ALL);
        frame.render_widget(timescale_block, top[0]);

        let change_block = Block::default()
            .title("Change");
        frame.render_widget(change_block, top[1]);

        let graph_block = Block::default()
             .title("Graph")
             .borders(Borders::ALL);
        frame.render_widget(graph_block, bottom[0]);

        let exchange_block = Block::default()
             .title("Exchange")
             .borders(Borders::ALL);

        frame.render_widget(exchange_block, bottom[1]);
    })?;

    Ok(())
}
