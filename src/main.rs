mod app;
mod chat;
mod model;
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

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let path_to_model = "mistral-7b-instruct-v0.2.Q5_K_M.gguf";
    let system = "You're a helpful chatbot that gives succint answers.";

    let llm = model::LLM::new(path_to_model, 0.9, 1.0, 512);
    let chat = chat::Chat::new(llm, system);
    //test(chat);
    //Ok(())
    let mut terminal = init()?;
    app::App::init(chat).run(&mut terminal)?;
    restore()
}

fn init() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    Terminal::with_options(
        backend,
        ratatui::TerminalOptions {
            viewport: ratatui::Viewport::Inline(40),
        },
    )
}

fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn test(mut chat: chat::Chat) {
    chat.add_user_msg("tell me (BRIEFLY!) about NYC");
    chat.add_assistant_msg("nyc is great");
    chat.add_user_msg("how come?");
    chat.add_assistant_msg("just is");
    chat.add_user_msg("give me lots of good examples");

    let response = chat.generate();
    chat.add_assistant_msg(&response);

    chat.add_user_msg("even more examples tell me a lot");

    let response2 = chat.generate();
    chat.add_assistant_msg(&response2);

    chat.clear();
}
