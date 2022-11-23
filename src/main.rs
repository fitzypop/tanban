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
    backend::{Backend, CrosstermBackend},
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

fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Board").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        thread::sleep(Duration::from_millis(5000));
        break;
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut backend = CrosstermBackend::new(stdout);

    // Set terminal title
    backend.execute(SetTitle("Tanban - Rust Kanban Board"))?;

    let mut terminal = Terminal::new(backend)?;

    // starts event loop
    run(&mut terminal)?;

    // restore terminal
    // todo! refactor this block into fn.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
