use crossterm::event::KeyEvent;
use ratatui::layout::Alignment;
use ratatui::prelude::Frame;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders};
use tui_textarea::{CursorMove, TextArea};

pub struct StatefulUI<'a> {
    text_area: TextArea<'a>,
}

impl<'a> StatefulUI<'a> {
    pub fn init() -> StatefulUI<'a> {
        let title = "Mistral Instruct";
        let controls =
            "Scroll: <PgUp/PgDn> | Submit: <Enter> | Clear: <Ctrl+X> | Copy latest output: <Ctrl+Y>";
        let exit = "Quit: <Esc>";

        let mut text_area = TextArea::default();
        text_area.set_placeholder_text("Enter prompt...");
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

    pub fn draw_ui(&self, frame: &mut Frame) {
        frame.render_widget(self.text_area.widget(), frame.size());
    }

    pub fn update_state(&mut self, key: KeyEvent) {
        self.text_area.input(key);
    }

    pub fn lines(&self) -> Vec<String> {
        self.text_area.lines().to_vec()
    }

    pub fn clear(&mut self) {
        self.text_area.move_cursor(CursorMove::End);
        self.text_area.delete_line_by_head();
    }

    //pub fn copy_latest_output(self) {
    //    self.
    //}
}
