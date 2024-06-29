use crossterm::event::KeyEvent;
use ratatui::layout::Alignment;
use ratatui::prelude::Frame;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders};
use tui_textarea::TextArea;

pub struct StatefulUI<'a> {
    text_area: TextArea<'a>,
}

impl<'a> StatefulUI<'a> {
    pub fn init() -> StatefulUI<'a> {
        let title = "Mistral Instruct";
        let controls =
            "Scroll: <PgUp/PgDn> | Submit: <Enter> | Clear: <Ctrl+X> | Copy cell: <Ctrl+Y>";
        let exit = "Quit: <Esc>";

        let mut text_area = TextArea::default();
        text_area.set_block(
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
        Self { text_area }
    }

    pub fn draw_ui(&mut self, frame: &mut Frame, maybe_key: Option<KeyEvent>) {
        if let Some(key) = maybe_key {
            self.text_area.input(key);
        }
        frame.render_widget(self.text_area.widget(), frame.size());
    }
}
