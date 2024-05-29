use ratatui::prelude::Frame;
use ratatui::widgets::{Block, Paragraph};

pub fn draw_ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
        frame.size(),
    );
}
