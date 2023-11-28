use anyhow::Result;
use crossterm::{
    event::{
        self,
        Event::Key,
        KeyCode::{Char, Down, Esc, Up},
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Frame, Stylize, Terminal},
    widgets::Paragraph,
};

struct App {
    counter: i64,
    should_quit: bool,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, frame: &mut Frame<'_>) {
    frame.render_widget(
        Paragraph::new(format!("Counter: {}", app.counter))
            .yellow()
            .on_black(),
        frame.size(),
    );
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Up => app.counter += 1,
                    Down => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    Esc => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    loop {
        terminal.draw(|frame| {
            ui(&app, frame);
        })?;

        update(&mut app)?;

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    startup()?;
    let result = run();
    shutdown()?;
    result?;
    Ok(())
}
