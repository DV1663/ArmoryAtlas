use std::io;

use crate::{search_items, ItemProduct};
use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use log::error;
use ratatui::layout::Constraint;
use ratatui::prelude::{Backend, CrosstermBackend, Style};
use ratatui::widgets::{Block, Borders, Row, Table};
use ratatui::Terminal;
use sqlx_mysql::MySqlPool;
use std::sync::{Arc, Mutex};
use tui_textarea::{Input, Key, TextArea};

use crate::tui::app::{App, CurrentScreen};
use crate::tui::key_events::screen_key_events;
use crate::tui::ui::ui;

mod app;
mod key_events;
mod ui;

pub async fn run_tui(pool: MySqlPool) -> Result<()> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture);

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(pool)?;
    let res = run_app(&mut terminal, &mut app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            todo!()
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

async fn fetch_data(app: &Arc<Mutex<App>>) -> Result<Vec<ItemProduct>> {
    let query = "
            SELECT
                i.ProductID as product_id,
                p.NameOfProduct AS product_name,
                p.Type AS product_type,
                i.Quantity as quantity,
                i.Size AS size
            FROM
                Products p
                    JOIN
                (SELECT ProductID, Size, count(*) as Quantity from Items group by ProductID, Size)
                    AS
                    i ON p.ProductID = i.ProductID
            WHERE
                i.Quantity > 0;
            ";

    let items: Vec<ItemProduct> = sqlx::query_as::<_, ItemProduct>(query)
        .fetch_all(&app.lock().unwrap().pool)
        .await?;

    Ok(items)
}

fn get_data(app: &mut App, items: &Option<Vec<ItemProduct>>) -> Result<Table<'static>> {
    if app.current_screen == CurrentScreen::Main {
        let mut rows = Vec::new();
        if let Some(page) = items {
            rows = page
                .iter()
                .map(|i| {
                    Row::new(vec![
                        i.product_id.clone(),
                        i.product_name.clone(),
                        i.product_type.clone(),
                        i.quantity.to_string(),
                        i.size.clone(),
                    ])
                })
                .collect();
        }

        let widths = [
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];

        let table = Table::new(rows, widths)
            .header(
                Row::new(vec![
                    "Product ID",
                    "Product Name",
                    "Product Type",
                    "Quantity",
                    "Size",
                ])
                .style(Style::default().add_modifier(ratatui::style::Modifier::BOLD)),
            )
            .block(
                Block::default()
                    .title("Items in Storage")
                    .borders(Borders::ALL),
            );

        Ok(table)
    } else {
        Ok(Table::default())
    }
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    let data: Vec<ItemProduct> = match app.db_handler.get_items() {
        Ok(data) => data,
        Err(err) => {
            error!("{err:?}");
            return Ok(false);
        }
    }
    .iter()
    .map(|item| item.into())
    .collect();

    // Store the iterator and its values in variables
    let data_iterator: Vec<Vec<ItemProduct>> =
        data.chunks(15).map(|chunk| chunk.to_vec()).collect();
    app.max_page = data_iterator.len();
    let mut data_to_display = data_iterator[0].clone();
    let mut search_box = TextArea::default();
    search_box.set_cursor_line_style(Style::default());
    search_box.set_placeholder_text("Item name or ID to Search for");

    loop {
        let table = match get_data(app, &Some(data_to_display.clone())) {
            Ok(table) => Some(table),
            Err(_) => None,
        };

        terminal.draw(|f| {
            ui(f, app, table, &mut search_box).expect("Error rendering the UI");
        })?;

        if let Event::Key(key) = event::read()? {
            match key.into() {
                Input {
                    key: Key::Enter, ..
                } => {
                    if app.current_screen == CurrentScreen::Main {
                        // search for the item either via name or id
                        let query = search_box.lines()[0].trim().to_string().replace(" ", "%");
                        if query.is_empty() {
                            data_to_display.clone_from(&data_iterator[app.current_page]);
                            continue;
                        }
                        // search database and displat the result
                        let search_result = search_items(&query).await;
                        match search_result {
                            Ok(items) => {
                                let items: Vec<ItemProduct> =
                                    items.iter().map(|item| item.into()).collect();
                                data_to_display.clone_from(&items);
                            }
                            Err(e) => {
                                error!("{:?}", e)
                            }
                        }
                    }
                }
                input => {
                    if app.current_screen == CurrentScreen::Main {
                        search_box.input(input);
                    }
                }
            }

            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            let (should_exit, new_data_to_display) =
                screen_key_events(app, key, &data_iterator, data_to_display);
            data_to_display = new_data_to_display;

            if should_exit {
                return Ok(false);
            }
        }
    }
}
