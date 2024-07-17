use crossterm::event::KeyEvent;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect, Size};
use ratatui::prelude::Frame;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use tui_scrollview::{ScrollView, ScrollViewState};
use tui_textarea::{CursorMove, TextArea};

pub struct UI<'a> {
    layout: Layout,
    scroll_view_state: ScrollViewState,
    text_area: TextArea<'a>,
    chat_frame: Block<'a>,
}

const SINGLE_OFFSET: u16 = 1;
const DOUBLE_OFFSET: u16 = SINGLE_OFFSET * 2;

impl<'a> UI<'a> {
    pub fn init() -> UI<'a> {
        let title = "Mistral Instruct";
        let controls = "Scroll: < ↑/↓ > | Submit: <Enter> | Clear: <Ctrl+X>";
        let exit = "Quit: <Esc>";

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(65), Constraint::Percentage(35)]);

        let mut text_area = TextArea::default();
        text_area.set_placeholder_text("Enter prompt...");
        text_area.set_block(
            Block::default()
                .title(
                    Title::from(controls)
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .borders(Borders::ALL),
        );

        let chat_frame = Block::default()
            .title(
                Title::from(title)
                    .alignment(Alignment::Center)
                    .position(Position::Top),
            )
            .title(
                Title::from(exit)
                    .alignment(Alignment::Left)
                    .position(Position::Top),
            )
            .borders(Borders::ALL);

        let scroll_view_state = ScrollViewState::default();

        Self {
            layout,
            scroll_view_state,
            text_area,
            chat_frame,
        }
    }

    fn build_paragaph(text: String, is_user: bool) -> Paragraph<'a> {
        let (style, title) = if is_user {
            (Style::new().blue(), "User")
        } else {
            (Style::new().yellow(), "Assistant")
        };
        Paragraph::new(text.to_string())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(style)
                    .title(
                        Title::from(title)
                            .alignment(Alignment::Center)
                            .position(Position::Top),
                    ),
            )
    }

    pub fn draw_ui(&mut self, frame: &mut Frame, message_history: &Vec<(String, String)>) {
        let split = self.layout.split(frame.size());

        let w = split[0].width;
        let h = split[0].height;
        let x = split[0].x;
        let y = split[0].y;

        let box_width = ((w as f32) * 0.6) as u16;
        let mut scroll_view = ScrollView::new(Size::new(w - DOUBLE_OFFSET - SINGLE_OFFSET, h * 2)); // FIXME: compute height

        let mut last_line = 0;
        for (user, assistant) in message_history.iter() {
            let lines_needed = Self::render_msg_bubble(
                user,
                true,
                &mut scroll_view,
                w - box_width - DOUBLE_OFFSET - DOUBLE_OFFSET,
                last_line,
                box_width,
            );
            last_line = last_line + lines_needed;

            let lines_needed = Self::render_msg_bubble(
                assistant,
                false,
                &mut scroll_view,
                DOUBLE_OFFSET,
                last_line,
                box_width,
            );
            last_line = last_line + lines_needed;
        }

        frame.render_widget(self.text_area.widget(), split[1]);
        frame.render_stateful_widget(
            scroll_view,
            Rect::new(x, y + SINGLE_OFFSET, w - DOUBLE_OFFSET, h - DOUBLE_OFFSET),
            &mut self.scroll_view_state,
        );
        frame.render_widget(self.chat_frame.clone(), split[0]);
    }

    fn render_msg_bubble(
        txt: &str,
        is_user: bool,
        scroll_view: &mut ScrollView,
        x: u16,
        y: u16,
        w: u16,
    ) -> u16 {
        let p = Self::build_paragaph(txt.to_string(), is_user);
        let lines_needed = p.line_count(w) as u16 + DOUBLE_OFFSET;
        scroll_view.render_widget(p, Rect::new(x, y, w, lines_needed));
        lines_needed
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

    pub fn lines(&self) -> String {
        self.text_area
            .lines()
            .iter()
            .fold("".to_string(), |acc, line| format!("{acc} {line}\n"))
    }

    pub fn clear_input(&mut self) {
        self.text_area.move_cursor(CursorMove::End);
        self.text_area.delete_line_by_head();
    }
}
