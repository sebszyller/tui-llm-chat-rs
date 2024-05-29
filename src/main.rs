mod events;
mod ui;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};

use std::io::{self, stdout, Stdout};

type Tui = Terminal<CrosstermBackend<Stdout>>;

fn main() -> io::Result<()> {
    let mut terminal = init()?;
    app_loop(&mut terminal)?;
    restore()
}

fn init() -> io::Result<Tui> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn app_loop(terminal: &mut Tui) -> io::Result<()> {
    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui::draw_ui)?;
        should_quit = events::handle_events()?;
    }
    Ok(())
}
