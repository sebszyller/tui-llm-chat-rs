use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;

pub enum UIEvent {
    Exit,
    Poll,
    Clear,
    PassKey(KeyEvent),
    CopyOutput,
    ScrollUp,
    ScrollDown,
    Submit,
}
pub fn handle_events() -> io::Result<UIEvent> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => return Ok(UIEvent::Exit),
                    KeyCode::Enter => return Ok(UIEvent::Submit),
                    KeyCode::PageUp => return Ok(UIEvent::ScrollUp),
                    KeyCode::PageDown => return Ok(UIEvent::ScrollDown),
                    KeyCode::Char(c) => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            match c {
                                'x' => return Ok(UIEvent::Clear),
                                'y' => return Ok(UIEvent::CopyOutput),
                                _ => return Ok(UIEvent::Poll),
                            }
                        } else {
                            return Ok(UIEvent::PassKey(key));
                        }
                    }
                    _ => return Ok(UIEvent::PassKey(key)),
                }
                //if key.code == KeyCode::Esc {
                //    return Ok(UIEvent::Exit);
                //} else if key.code == KeyCode::Enter {
                //    return Ok(UIEvent::Submit);
                //} else if key.modifiers.contains(KeyModifiers::CONTROL) {
                //    match key.code {
                //        KeyCode::Char('x') => return Ok(UIEvent::Clear),
                //        KeyCode::Char('y') => return Ok(UIEvent::CopyOutput),
                //        _ => return Ok(UIEvent::Poll),
                //    }
                //} else {
                //    return Ok(UIEvent::PassKey(key));
                //}
            }
        }
    }
    Ok(UIEvent::Poll)
}
