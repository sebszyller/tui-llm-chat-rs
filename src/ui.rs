use crossterm::event::KeyEvent;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect, Size};
use ratatui::prelude::Frame;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use tui_scrollview::{ScrollView, ScrollViewState};
use tui_textarea::{CursorMove, TextArea};

//https://github.com/joshka/tui-scrollview/blob/main/examples/scrollview.rs

pub enum Side {
    Left,
    Right,
}

pub struct StatefulUI<'a> {
    output_lines: Vec<(String, Side)>,
    layout: Layout,
    scroll_view_state: ScrollViewState,
    text_area: TextArea<'a>,
}

const SINGLE_OFFSET: u16 = 1;
const DOUBLE_OFFSET: u16 = SINGLE_OFFSET * 2;

impl<'a> StatefulUI<'a> {
    pub fn init() -> StatefulUI<'a> {
        let title = "Mistral Instruct";
        let controls =
            "Scroll: <PgUp/PgDn> | Submit: <Enter> | Clear: <Ctrl+X> | Copy latest output: <Ctrl+Y>";
        let exit = "Quit: <Esc>";

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(65), Constraint::Percentage(35)]);

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

        let scroll_view_state = ScrollViewState::default();
        let output_lines: Vec<(String, Side)> = vec![];

        Self {
            output_lines,
            layout,
            scroll_view_state,
            text_area,
        }
    }

    fn build_paragaph(text: String) -> Paragraph<'a> {
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false })
    }

    pub fn draw_ui(&mut self, frame: &mut Frame) {
        let split = &self.layout.split(frame.size());

        let w = split[0].width;
        let h = split[0].height;
        let x = split[0].x;
        let y = split[0].y;

        let box_width = ((w as f32) * 0.6) as u16;
        let mut scroll_view = ScrollView::new(Size::new(w - DOUBLE_OFFSET, h * 10));

        let mut last_line = 0;
        for (txt, side) in self.output_lines.iter() {
            let p = StatefulUI::build_paragaph(txt.to_string());
            let lines_needed = p.line_count(box_width) as u16 + DOUBLE_OFFSET;
            let xp = match side {
                Side::Left => SINGLE_OFFSET,
                Side::Right => w - box_width - DOUBLE_OFFSET - SINGLE_OFFSET,
            };
            scroll_view.render_widget(p, Rect::new(xp, last_line, box_width, lines_needed));
            last_line = last_line + lines_needed;
        }
        frame.render_widget(self.text_area.widget(), split[1]);
        frame.render_stateful_widget(
            scroll_view,
            Rect::new(x, y + SINGLE_OFFSET, w - SINGLE_OFFSET, h - DOUBLE_OFFSET),
            &mut self.scroll_view_state,
        );
        frame.render_widget(Block::default().borders(Borders::all()), split[0]);
    }

    pub fn update_text_area_state(&mut self, key: KeyEvent) {
        self.text_area.input(key);
    }

    pub fn scroll_up(&mut self) {
        self.scroll_view_state.scroll_page_up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll_view_state.scroll_page_down();
    }

    pub fn add_output_lines(&mut self, text: &str, side: Side) {
        self.output_lines.push((text.to_string(), side));
    }

    pub fn lines(&self) -> Vec<String> {
        self.text_area.lines().to_vec()
    }

    pub fn clear_input(&mut self) {
        self.text_area.move_cursor(CursorMove::End);
        self.text_area.delete_line_by_head();
    }

    pub fn clear_output(&mut self) {
        self.output_lines = vec![];
    }

    pub fn copy_latest_output(&self) {
        _ = "this is the latest output".to_string();
    }
}
