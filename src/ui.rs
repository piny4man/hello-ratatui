use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!(
            "
EXIT -> `Esc` | `Ctrl-C` | `q` \n
INCREMENT -> `UP` \n
DECREMENT -> `DOWN` \n
--------------------------------- \n
Counter: {}
            ",
            app.counter
        ))
        .block(
            Block::default()
                .title("  Sync Counter TUI  ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
        )
        .style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
        .alignment(Alignment::Center),
        frame.size(),
    )
}
