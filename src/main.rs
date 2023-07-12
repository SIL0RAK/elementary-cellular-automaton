use std::{
    io,
    thread,
    time::Duration,
    time::Instant,
    sync::mpsc,
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, canvas::{Canvas, Rectangle, Map, MapResolution, Line}},
    layout::{Layout, Constraint, Direction, Alignment},
    Terminal, style::{Color}
};
use crossterm::{
    event::{self, EnableMouseCapture, Event as CEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    // Input handling
    thread::spawn(move || {
        let mut start: Instant = Instant::now();

        loop {
            let timeout: Duration = tick_rate
                .checked_sub(start.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if start.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    start = Instant::now();
                }
            }
        }
    });

    let mut current_row = 0;

    // Draw loop
    loop {
        terminal.draw(|rect| {
            let size = rect.size();

            // Layout chunks
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(30),
                        Constraint::Min(40),
                    ]
                    .as_ref(),
                )
                .split(size);

            let options_block = Block::default()
                .title("Options")
                .borders(Borders::ALL);

            let canvas_block = Canvas::default()
                .block(Block::default().title("Canvas").borders(Borders::ALL))
                .x_bounds([0.0, 200.0])
                .y_bounds([0.0, 100.0])
                .paint(|ctx| {
                    ctx.layer();

                    for y in 1..100 {
                        for x in 1..200 {
                            // TODO use points
                            ctx.draw(&Line {
                                x1: x as f64,
                                y1: y as f64,
                                x2: x as f64,
                                y2: y as f64,
                                color: Color::White,
                            });
                        }
                    }
                });

            rect.render_widget(options_block, chunks[0]);
            rect.render_widget(canvas_block, chunks[1]);

        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                event::KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                },
                _ => {}
            }
            Event::Tick => {}
        }
    }

    Ok(())
}