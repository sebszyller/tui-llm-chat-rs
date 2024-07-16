mod app;
mod model;
mod state;
mod ui;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use env_logger;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{self, stdout, Stdout};
use tracing;
use tracing_subscriber::FmtSubscriber;

fn main() -> io::Result<()> {
    env_logger::init();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::ERROR)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting the default subscriber failed!");

    let path_to_model = "mistral-7b-instruct-v0.2.Q5_K_M.gguf";
    let system = "You're a helpful chatbot that gives succint answers.";

    let llm = model::LLM::new(path_to_model, system, 0.9, 1.0, 2048);
    let state = state::State::new();
    let mut terminal = init()?;
    app::App::init(state, llm).run(&mut terminal)?;
    restore()
}

fn init() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    Terminal::with_options(
        backend,
        ratatui::TerminalOptions {
            viewport: ratatui::Viewport::Fullscreen,
        },
    )
}

fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
