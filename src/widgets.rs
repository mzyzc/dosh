use tui::widgets::{Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

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

pub fn get_timescale_block<>() -> Block<'static> {
    Block::default()
        .title("Timescale")
        .borders(Borders::ALL)
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