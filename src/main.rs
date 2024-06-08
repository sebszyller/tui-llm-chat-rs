mod chat;

//use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
//use crossterm::terminal::{
//    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
//};
//use ratatui::backend::CrosstermBackend;
//use ratatui::widgets::{Block, Borders};
//use ratatui::Terminal;
//use std::io;
//use tui_textarea::{Input, Key, TextArea};

fn main() {
    let llm_chat = chat::Chat::new(19);
    println!("{}", llm_chat.generate());
}
//fn main() -> io::Result<()> {
//let stdout = io::stdout();
//let mut stdout = stdout.lock();
//
//enable_raw_mode()?;
//crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//let backend = CrosstermBackend::new(stdout);
//let mut term = Terminal::new(backend)?;
//
//let mut textarea = TextArea::default();
//textarea.set_block(
//    Block::default()
//        .borders(Borders::ALL)
//        .title("Crossterm Minimal Example"),
//);
//
//loop {
//    term.draw(|f| {
//        f.render_widget(textarea.widget(), f.size());
//    })?;
//    match crossterm::event::read()?.into() {
//        Input { key: Key::Esc, .. } => break,
//        input => {
//            textarea.input(input);
//        }
//    }
//}
//
//disable_raw_mode()?;
//crossterm::execute!(
//    term.backend_mut(),
//    LeaveAlternateScreen,
//    DisableMouseCapture
//)?;
//term.show_cursor()?;
//
//println!("Lines: {:?}", textarea.lines());
//Ok(())
//}
//mod events;
//mod ui;
//
//use crossterm::{
//    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//    ExecutableCommand,
//};
//use ratatui::prelude::{CrosstermBackend, Terminal};
//
//use std::io::{self, stdout, Stdout};
//
//type Tui = Terminal<CrosstermBackend<Stdout>>;
//
//fn main() -> io::Result<()> {
//    let mut terminal = init()?;
//    app_loop(&mut terminal)?;
//    restore()
//}
//
//fn init() -> io::Result<Tui> {
//    enable_raw_mode()?;
//    stdout().execute(EnterAlternateScreen)?;
//    let backend = CrosstermBackend::new(stdout());
//    Terminal::with_options(
//        backend,
//        ratatui::TerminalOptions {
//            viewport: ratatui::Viewport::Inline(20),
//        },
//    )
//}
//
//fn restore() -> io::Result<()> {
//    disable_raw_mode()?;
//    stdout().execute(LeaveAlternateScreen)?;
//    Ok(())
//}
//
//fn app_loop(terminal: &mut Tui) -> io::Result<()> {
//    let mut should_quit = false;
//    while !should_quit {
//        terminal.draw(ui::draw_ui)?;
//        should_quit = events::handle_events()?;
//    }
//    Ok(())
//}
