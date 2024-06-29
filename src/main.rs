mod chat;
mod events;
mod model;
mod ui;

use crossterm::{
    event::KeyEvent,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use env_logger;
use ratatui::prelude::{CrosstermBackend, Terminal};

use std::io::{self, stdout, Stdout};

type Tui = Terminal<CrosstermBackend<Stdout>>;

fn main() -> io::Result<()> {
    env_logger::init();

    let path_to_model = "mistral-7b-instruct-v0.2.Q5_K_M.gguf";
    let system = "You're a helpful chatbot that gives succint answers.";

    let mut llm = model::LLM::new(path_to_model, 0.9, 1.0, 512);
    let chat = chat::Chat::new(&mut llm, system);
    //test(chat);

    let mut terminal = init()?;
    app_loop(&mut terminal)?;
    restore()
}

fn init() -> io::Result<Tui> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    Terminal::with_options(
        backend,
        ratatui::TerminalOptions {
            viewport: ratatui::Viewport::Inline(20),
        },
    )
}

fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn app_loop(terminal: &mut Tui) -> io::Result<()> {
    let mut should_quit = false;
    let mut maybe_key: Option<KeyEvent> = None;
    while !should_quit {
        (maybe_key, should_quit) = events::handle_events()?;
        terminal.draw(|frame| ui::draw_ui(frame, maybe_key));
    }
    Ok(())
}

fn test(mut chat: chat::Chat) {
    chat.add_user_msg("tell me (BRIEFLY!) about NYC");
    chat.add_assistant_msg("nyc is great");
    chat.add_user_msg("how come?");
    chat.add_assistant_msg("just is");
    chat.add_user_msg("give me one good example");

    let response = chat.generate();
    chat.add_assistant_msg(&response);

    chat.clear();
}
