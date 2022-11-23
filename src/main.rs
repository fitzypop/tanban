use chrono::{DateTime, Utc};
use crossterm::terminal::enable_raw_mode;
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Debug, Clone)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Default for Status {
    fn default() -> Self {
        Self::Todo
    }
}

#[derive(Debug, Default, Clone)]
struct Card {
    title: String,
    description: String,
    created: DateTime<Utc>,
    status: Status,
}

#[derive(Debug, Default)]
struct AppState {
    cards: Vec<Card>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cards: vec![Card::default(); 2],
        }
    }
}
fn main() -> Result<(), io::Error> {
    enable_raw_mode().unwrap();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    // idk wtf im doing, and im tired af
    let state = AppState::new();
    println!("{:?}", state);
    Ok(())
}
