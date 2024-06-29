use crossterm::event::KeyEvent;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::Frame;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders, Paragraph};
use tui_textarea::{CursorMove, TextArea};

pub struct StatefulUI<'a> {
    layout: Layout,
    output: Paragraph<'a>,
    text_area: TextArea<'a>,
}

impl<'a> StatefulUI<'a> {
    pub fn init() -> StatefulUI<'a> {
        let title = "Mistral Instruct";
        let controls =
            "Scroll: <PgUp/PgDn> | Submit: <Enter> | Clear: <Ctrl+X> | Copy latest output: <Ctrl+Y>";
        let exit = "Quit: <Esc>";

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)]);
        //.split(frame.size());

        let output = StatefulUI::build_paragaph("Go on bruv...".to_string());
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
        Self {
            layout,
            output,
            text_area,
        }
    }

    fn build_paragaph(text: String) -> Paragraph<'a> {
        Paragraph::new(text).block(Block::default().borders(Borders::ALL))
    }

    pub fn draw_ui(&self, frame: &mut Frame) {
        let split = self.layout.split(frame.size());
        frame.render_widget(self.output.clone(), split[0]);
        frame.render_widget(self.text_area.widget(), split[1]);
    }

    pub fn update_text_area_state(&mut self, key: KeyEvent) {
        self.text_area.input(key);
    }

    pub fn update_output_state(&mut self, new_output: String) {
        self.output = StatefulUI::build_paragaph(new_output);
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
