use anyhow::Result;
use ratatui::prelude::{Color, Constraint, Direction, Layout, Line, Rect, Span, Style, Text};
use ratatui::widgets::Block;
use ratatui::widgets::{Borders, Paragraph, Table};
use ratatui::Frame;
use tui_textarea::TextArea;

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

pub fn ui(
    f: &mut Frame<'_>,
    app: &App,
    main_page_data: Option<Table>,
    search_box: &mut TextArea,
) -> Result<()> {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let content_container = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(main_layout[1]);

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(content_container[1]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "ArmoryAtlas",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, main_layout[0]);

    if app.current_screen == CurrentScreen::Main {
        // render a search bar with a header for the page
        f.render_widget(search_box.widget(), content_container[0]);

        if let Some(new_data) = main_page_data {
            f.render_widget(new_data, content_layout[1]);
        }
    }

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                format!(
                    "(q) to quit / (esc) to enter settings / <-- {}/{} -->",
                    app.current_page + 1,
                    app.max_page
                ),
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Settings => Span::styled(
                "(c) to edit the config / (q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exit => Span::styled(
                "(y) to quit / (n) to go back",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Config => Span::default(),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[2]);

    f.render_widget(key_notes_footer, footer_chunks[1]);

    Ok(())
}

pub fn render_config_page(f: &mut Frame<'_>, app: &App) -> Result<()> {
    
    
    
    Ok(())
}
