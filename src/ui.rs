use ratatui::layout::Alignment;
use ratatui::prelude::Frame;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders};

pub fn draw_ui(frame: &mut Frame) {
    let widget = Block::default()
        .title(
            Title::from("Score: 96 | Best: 4096")
                .alignment(Alignment::Center)
                .position(Position::Top),
        )
        .title(
            Title::from("Navigation: H/J/K/L")
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .title(
            Title::from("[Q]uit")
                .alignment(Alignment::Left)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL);
    frame.render_widget(widget, frame.size());
}
