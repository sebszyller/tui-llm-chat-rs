use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

pub fn handle_events() -> io::Result<(Option<KeyEvent>, bool)> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Esc {
                return Ok((None, true));
            } else {
                return Ok((Some(key), false));
            }
        }
    }
    Ok((None, false))
}
