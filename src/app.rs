use crate::model;
use crate::state;
use crate::ui;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use llama_cpp::CompletionHandle;
use llama_cpp::TokensToStrings;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

use std::io::Stdout;

pub enum AppEvent {
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
    state: state::State,
    llm: model::LLM,
    ui: ui::UI<'a>,
}

impl<'a> App<'a> {
    pub fn init(state: state::State, llm: model::LLM) -> App<'a> {
        Self {
            state,
            llm,
            ui: ui::UI::init(),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        let mut generating = false;
        let mut completions: Option<TokensToStrings<CompletionHandle>> = None;
        loop {
            let _ = terminal.draw(|frame| self.ui.draw_ui(frame, &self.state.message_history));
            if generating {
                if let Some(ref mut comp) = completions {
                    match comp.next() {
                        Some(partial_output) => self.state.add_assistant_msg(&partial_output),
                        None => generating = false,
                    }
                } else {
                    self.state
                        .add_assistant_msg("Context is full; clear to continue!");
                    generating = false
                }
            } else {
                match Self::key_as_event()? {
                    AppEvent::Exit => break,
                    AppEvent::Idle => continue,
                    AppEvent::Clear => {
                        self.llm.clear_session().expect("Failed to clear session!");
                        self.state.clear();
                        self.ui.clear_input();
                    }
                    AppEvent::KeyInput(key) => self.ui.update_text_area_state(key),
                    AppEvent::Submit => {
                        let input = self.ui.lines();
                        self.ui.clear_input();
                        self.state.add_user_msg(&input);
                        completions = self.start_stream();
                        generating = true;
                    }
                    AppEvent::CopyOutput => self.ui.copy_latest_output(),
                    AppEvent::ScrollUp => self.ui.scroll_up(),
                    AppEvent::ScrollDown => self.ui.scroll_down(),
                }
            }
        }
        Ok(())
    }

    fn start_stream(&mut self) -> Option<TokensToStrings<CompletionHandle>> {
        self.llm
            .prepare_completion_handle(&self.state.message_history)
            .ok()
    }

    fn key_as_event() -> io::Result<AppEvent> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => return Ok(AppEvent::Exit),
                        KeyCode::Enter => return Ok(AppEvent::Submit),
                        KeyCode::Up => return Ok(AppEvent::ScrollUp),
                        KeyCode::Down => return Ok(AppEvent::ScrollDown),
                        KeyCode::Char(c) => {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                match c {
                                    'x' => return Ok(AppEvent::Clear),
                                    'y' => return Ok(AppEvent::CopyOutput),
                                    _ => return Ok(AppEvent::Idle),
                                }
                            } else {
                                return Ok(AppEvent::KeyInput(key));
                            }
                        }
                        _ => return Ok(AppEvent::KeyInput(key)),
                    }
                }
            }
        }
        Ok(AppEvent::Idle)
    }
}
