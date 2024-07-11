use crate::chat;
use crate::ui;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

use std::io::Stdout;

pub enum UIEvent {
    Exit,
    Idle,
    Clear,
    KeyInput(KeyEvent),
    CopyOutput,
    ScrollUp,
    ScrollDown,
    Submit,
}

pub struct App<'a> {
    chat: chat::Chat,
    stateful_ui: ui::StatefulUI<'a>,
}

impl<'a> App<'a> {
    pub fn init(chat: chat::Chat) -> App<'a> {
        Self {
            chat,
            stateful_ui: ui::StatefulUI::init(),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        loop {
            let _ = terminal.draw(|frame| self.stateful_ui.draw_ui(frame));
            match App::handle_events()? {
                UIEvent::Exit => break,
                UIEvent::Idle => continue,
                UIEvent::Clear => {
                    self.chat.clear();
                    self.stateful_ui.clear_input();
                    self.stateful_ui.clear_output();
                }
                UIEvent::KeyInput(key) => self.stateful_ui.update_text_area_state(key),
                UIEvent::Submit => {
                    let lines = self.stateful_ui.lines();
                    self.stateful_ui.clear_input();
                    self.progress_chat(&lines);
                }
                UIEvent::CopyOutput => self.stateful_ui.copy_latest_output(),
                UIEvent::ScrollUp => self.stateful_ui.scroll_up(),
                UIEvent::ScrollDown => self.stateful_ui.scroll_down(),
            }
        }
        Ok(())
    }

    fn progress_chat(&mut self, lines: &Vec<String>) {
        let input = lines
            .iter()
            .fold("".to_string(), |acc, line| format!("{acc} {line}\n"));

        self.chat.add_user_msg(&input);
        self.stateful_ui.add_output_lines(&input, ui::Side::Right);
        let output = self.chat.generate();
        self.chat.add_assistant_msg(&output);
        self.stateful_ui.add_output_lines(&output, ui::Side::Left);
    }

    fn handle_events() -> io::Result<UIEvent> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => return Ok(UIEvent::Exit),
                        KeyCode::Enter => return Ok(UIEvent::Submit),
                        KeyCode::Up => return Ok(UIEvent::ScrollUp),
                        KeyCode::Down => return Ok(UIEvent::ScrollDown),
                        KeyCode::Char(c) => {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                match c {
                                    'x' => return Ok(UIEvent::Clear),
                                    'y' => return Ok(UIEvent::CopyOutput),
                                    _ => return Ok(UIEvent::Idle),
                                }
                            } else {
                                return Ok(UIEvent::KeyInput(key));
                            }
                        }
                        _ => return Ok(UIEvent::KeyInput(key)),
                    }
                }
            }
        }
        Ok(UIEvent::Idle)
    }
}
