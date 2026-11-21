use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;
/// the actual drawer with all related stuff
pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `right` and `left` to increment and decrement the counter respectively.\n\
        Counter: {}
        ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter app")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        ,
        frame.area(),
    );
}
