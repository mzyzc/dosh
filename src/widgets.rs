use crate::coin::Coin;

use tui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Tabs};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Style};
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

    let titles = ["1D", "2D", "7D", "30D", "365D"].iter().cloned().map(Spans::from).collect();
    Tabs::new(titles)
        .block(block)
}

pub fn get_change_block(coin: &Coin) -> Paragraph {
    let block = Block::default()
        .title("Change");

    let increase = if *&coin.change > 0.0 {true} else {false};

    let text = format!("{}{:.2}%",
        if increase {"+"} else {""},
        *&coin.change,
    );

    Paragraph::new(text)
        .block(block)
        .style(Style::default()
            .fg(
                if increase {Color::Green} else if *&coin.change < 0.0 {Color::Red} else {Color::Reset}
            )
        )
}
pub fn get_graph(coin: &Coin) -> Chart {
    let datasets = vec![
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default()
                .fg(
                    if *&coin.change > 0.0 {Color::Green} else if *&coin.change < 0.0 {Color::Red} else {Color::Reset}
                )
            )
            .data(&coin.data_points)
    ];

    let block = Block::default()
        .title("History")
        .borders(Borders::ALL);

    let date_bounds = coin.get_date_bounds();
    let price_bounds = coin.get_price_bounds();

    Chart::new(datasets)
        .block(block)
        .x_axis(
            Axis::default()
                .bounds([date_bounds.0, date_bounds.1])
        )
        .y_axis(
            Axis::default()
                .bounds([price_bounds.0, price_bounds.1])
        )
}

pub fn get_exchange_block(coin: &Coin) -> Paragraph {
    let block = Block::default()
        .title("Exchange")
        .borders(Borders::ALL);

    let mut text: Vec<Spans> = coin.price
        .iter()
        .map(|e| Spans::from(format!("= {:.2} {}", e.value * &coin.quantity, e.currency.to_uppercase())))
        .collect();

    text.insert(
        0,
        Spans::from(format!("{} {}", &coin.quantity, &coin.name.to_uppercase())),
    );

    Paragraph::new(text)
        .block(block)
}