use std::cmp::PartialEq;
use ratatui::widgets::Block;
use ratatui::Frame;
use ratatui::prelude::{Color,
Constraint, Direction, Layout, Line, Rect, Span, Style, Text};
use ratatui::widgets::{Borders, ListItem, Paragraph, Row, Table};
use crate::ItemProduct;
use crate::tui::app::{App, CurrentScreen};
use anyhow::Result;

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

impl PartialEq for CurrentScreen {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

pub async fn ui(f: &mut Frame<'_>, app: &App) -> Result<()> {
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

    if app.current_screen == CurrentScreen::Main {
        let query = "
            SELECT Items.ItemID as item_id,
                   Products.NameOfProduct as name_of_product,
                   Products.Type as type_of_product,
                   Items.Size as size,
                   Items.LevelOfUse as level_of_use
            FROM Items
            INNER JOIN Products ON Items.ProductID = Products.ProductID
            LIMIT 10
        ";

        let items: Vec<ItemProduct> = sqlx::query_as::<_, ItemProduct>(query).fetch_all(&app.pool).await?;

        let rows: Vec<_> = items.iter().map(|i| {
            Row::new(vec![
                i.item_id.to_string(),
                i.name_of_product.clone(),
                i.type_of_product.clone(),
                i.size.clone(),
                i.level_of_use.clone(),
            ])
        }).collect();

        let widths = [
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];

        let table = Table::new(rows, widths)
            .header(Row::new(vec!["ID", "Product Name", "Type", "Size", "Level of Use"])
                .style(Style::default().add_modifier(ratatui::style::Modifier::BOLD)))
            .block(Block::default().title("Item Details").borders(Borders::ALL));

        f.render_widget(table, chunks[1]);
    }

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (esc) to enter settings / () to switch to previous page / () to switch to next page",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Settings => Span::styled(
                "(c) to edit the config / (q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exit => Span::styled(
                "(q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Config => {Span::default()}
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(key_notes_footer, footer_chunks[1]);

    Ok(())
}