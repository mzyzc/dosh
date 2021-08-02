use tui::widgets::{Block, Borders, Tabs};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::Spans;

pub fn get_chunks() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ].as_ref()
        )
}

pub fn get_top_chunk() -> Layout {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ].as_ref()
        )
}

pub fn get_bottom_chunk() -> Layout {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ].as_ref()
        )
}

pub fn get_timescale_tabs() -> Tabs<'static> {
    let block = Block::default()
        .title("Timescale")
        .borders(Borders::ALL);

    let titles = ["24H", "48H", "7D", "30D", "12M"].iter().cloned().map(Spans::from).collect();
    Tabs::new(titles)
        .block(block)
}

pub fn get_change_block() -> Block<'static> {
    Block::default()
        .title("Change")
}
pub fn get_graph_block() -> Block<'static> {
    Block::default()
        .title("Graph")
        .borders(Borders::ALL)
}

pub fn get_exchange_block() -> Block<'static> {
    Block::default()
        .title("Exchange")
        .borders(Borders::ALL)
}