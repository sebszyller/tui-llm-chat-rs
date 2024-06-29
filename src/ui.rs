use crossterm::event::KeyEvent;
use ratatui::layout::Alignment;
use ratatui::prelude::Frame;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders};
use tui_textarea::{Input, Key, TextArea};

pub fn draw_ui(frame: &mut Frame, maybe_key: Option<KeyEvent>) {
    let title = "Mistral Instruct";
    let controls = "Scroll: <PgUp/PgDn> | Submit: <Enter> | Clear: <Ctrl+X> | Copy cell: <Ctrl+Y>";
    let exit = "Quit: <Esc>";
    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .title(
                Title::from(title)
                    .alignment(Alignment::Center)
                    .position(Position::Top),
            )
            .title(
                Title::from(controls)
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .title(
                Title::from(exit)
                    .alignment(Alignment::Left)
                    .position(Position::Top),
            )
            .borders(Borders::ALL),
    );
    if let Some(key) = maybe_key {
        textarea.input(key);
    }
    frame.render_widget(textarea.widget(), frame.size());
}
