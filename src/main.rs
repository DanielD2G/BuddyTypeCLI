mod app;
mod config;
mod data;
mod engine;
mod screens;
mod types;
mod ui;

use app::App;
use crossterm::event::{self, Event};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Ensure cleanup on panic
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
        default_hook(info);
    }));

    // Run app
    let result = run_app(&mut terminal);

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();

    // Get initial terminal size
    let (w, h) = terminal::size()?;
    app.handle_resize(w, h);

    loop {
        terminal.draw(|frame| app.render(frame))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    // Ignore key release events (crossterm on some platforms)
                    if key.kind == crossterm::event::KeyEventKind::Release {
                        continue;
                    }
                    app.handle_key(key);
                }
                Event::Resize(w, h) => app.handle_resize(w, h),
                _ => {}
            }
        }

        app.tick();

        if app.should_quit {
            return Ok(());
        }
    }
}
