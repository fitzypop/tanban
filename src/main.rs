#[allow(unused_variables)]
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let _terminal = Terminal::new(backend)?;
    Ok(())
}
