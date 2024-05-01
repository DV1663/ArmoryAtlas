use anyhow::Result;
use ratatui::prelude::{Color, Constraint, Direction, Layout, Line, Rect, Span, Style, Text};
use ratatui::widgets::Block;
use ratatui::widgets::{Borders, Paragraph, Table};
use ratatui::Frame;

use crate::tui::app::{App, CurrentScreen};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn ui(f: &mut Frame<'_>, app: &App, data: Option<Table>) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "ArmoryAtlas",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    if let Some(new_data) = data {
        f.render_widget(new_data, chunks[1]);
    }

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                format!(
                    "(q) to quit / (esc) to enter settings / <-- (a) {}/{} (d) -->",
                    app.current_page + 1,
                    app.max_page
                ),
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Settings => Span::styled(
                "(c) to edit the config / (q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exit => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
            CurrentScreen::Config => Span::default(),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(key_notes_footer, footer_chunks[1]);

    Ok(())
}
