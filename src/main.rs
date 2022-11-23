use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
    ExecutableCommand,
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

// use chrono::{DateTime, Utc};

// #[derive(Debug, Default, Clone)]
// enum Status {
//     #[default]
//     Todo,
//     InProgress,
//     Done,
// }

// #[derive(Debug, Default, Clone)]
// struct Card {
//     title: String,
//     description: String,
//     created: DateTime<Utc>,
//     status: Status,
// }

// #[derive(Debug, Default)]
// struct AppState {
//     cards: Vec<Card>,
// }

// impl AppState {
//     pub fn new() -> Self {
//         Self {
//             cards: vec![Card::default(); 2],
//         }
//     }
// }

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut backend = CrosstermBackend::new(stdout);
    backend.execute(SetTitle("Tanban - Rust Kanban Board"))?;

    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
