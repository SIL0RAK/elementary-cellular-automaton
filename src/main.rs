use std::{
    io,
    thread,
    time::Duration,
    time::Instant,
    sync::mpsc,
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph, BorderType},
    layout::{Layout, Constraint, Direction, Alignment},
    Terminal, text::{Span, Spans}, style::{Style, Color}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                    ]
                    .as_ref(),
                )
                .split(size);

            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);

            let options_block = Block::default()
                .title("Block")
                .borders(Borders::ALL);


            rect.render_widget(block, chunks[0]);
            rect.render_widget(options_block, chunks[1]);
        })?;
    }

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