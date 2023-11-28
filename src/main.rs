use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut counter = 0;

    loop {
        terminal.draw(|frame| {
            frame.render_widget(
                Paragraph::new(format!("Counter: {counter}"))
                    .yellow()
                    .on_black(),
                frame.size(),
            );
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Up => counter += 1,
                        crossterm::event::KeyCode::Down => counter -= 1,
                        crossterm::event::KeyCode::Char('q') => break,
                        crossterm::event::KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }
    }

    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
