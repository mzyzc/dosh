use crate::coin::Coin;

use tui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Tabs};
use tui::layout::{Layout, Constraint, Direction};
use tui::symbols::Marker;
use tui::text::Spans;

pub fn get_chunks() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
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
pub fn get_graph(coin: &Coin) -> Chart {
    let datasets = vec![
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .data(&coin.data_points)
    ];

    let block = Block::default()
        .title("History")
        .borders(Borders::ALL);

    Chart::new(datasets)
        .block(block)
        .x_axis(
            Axis::default()
                .bounds([1627603200000.0, 1628159450000.0])
        )
        .y_axis(
            Axis::default()
                .bounds([1700.0, 1900.0])
        )
}

pub fn get_exchange_block() -> Block<'static> {
    Block::default()
        .title("Exchange")
        .borders(Borders::ALL)
}