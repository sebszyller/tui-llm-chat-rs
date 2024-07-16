mod app;
mod model;
mod state;
mod ui;

use clap::{Arg, ArgMatches, Command};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use env_logger;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{self, stdout, Stdout};
use tracing;
use tracing_subscriber::{filter::EnvFilter, fmt};

fn main() -> io::Result<()> {
    env_logger::init();
    let subscriber = if let Ok(env_filter) = EnvFilter::try_from_default_env() {
        fmt().with_env_filter(env_filter).finish()
    } else {
        fmt().with_env_filter(EnvFilter::new("off")).finish()
    };

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting the default subscriber failed!");

    let args: ArgMatches = parse_args().get_matches();
    let temperature: &f32 = &args.get_one::<f32>("temperature").unwrap_or(&0.9);
    let top_p: &f32 = &args.get_one::<f32>("top_p").unwrap_or(&1.0);
    let max_new_tokens: &usize = &args.get_one::<usize>("max_new_tokens").unwrap_or(&2048);
    let inline_lines: &u16 = &args.get_one::<u16>("inline_lines").unwrap_or(&40);

    let path_to_model = "mistral-7b-instruct-v0.2.Q5_K_M.gguf";
    let system = "You're a helpful chatbot that gives succint answers.";

    let llm = model::LLM::new(path_to_model, system, *temperature, *top_p, *max_new_tokens);
    let state = state::State::new();
    let mut terminal = init(*inline_lines)?;
    app::App::init(state, llm).run(&mut terminal)?;
    restore()
}

fn parse_args() -> Command {
    Command::new("TUI Chat")
        .version("0.1.0")
        .author("Sebastian Szyller")
        .about("Learning Rust with LLMs Capstone Project: TUI LLM Chat")
        .arg(
            Arg::new("temperature")
                .required(false)
                .long("temperature")
                .value_name("TEMPERATURE")
                .value_parser(clap::value_parser!(f32))
                .help("Sampling temperature"),
        )
        .arg(
            Arg::new("top_p")
                .required(false)
                .long("top_p")
                .value_name("TOP_P")
                .value_parser(clap::value_parser!(f32))
                .help("Use top_p tokens"),
        )
        .arg(
            Arg::new("max_new_tokens")
                .required(false)
                .long("max_new_tokens")
                .value_name("MAX_NEW_TOKENS")
                .value_parser(clap::value_parser!(usize))
                .help("Generate at most max_new_tokens"),
        )
        .arg(
            Arg::new("inline_lines")
                .required(false)
                .long("inline_lines")
                .value_name("INLINE_LINES")
                .value_parser(clap::value_parser!(u16))
                .help("Number of lines for the TUI"),
        )
}

fn init(inline_lines: u16) -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    Terminal::with_options(
        backend,
        ratatui::TerminalOptions {
            viewport: ratatui::Viewport::Inline(inline_lines),
        },
    )
}

fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
