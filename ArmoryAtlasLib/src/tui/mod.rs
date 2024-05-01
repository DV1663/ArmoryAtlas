use std::io;

use anyhow::Result;
use crossterm::{event, execute};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::Constraint;
use ratatui::prelude::{Backend, CrosstermBackend, Style};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Row, Table};
use sqlx_mysql::MySqlPool;
use crate::ItemProduct;
use std::sync::{Arc, Mutex};
use log::info;

use crate::tui::app::{App, CurrentScreen};
use crate::tui::ui::ui;

mod ui;
mod app;

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
                    Items.ItemID AS item_id,
                    Products.NameOfProduct AS name_of_product,
                    Products.Type AS type_of_product,
                    Items.Size AS size,
                    Items.LevelOfUse AS level_of_use
                FROM
                    Items
                INNER JOIN
                    Products ON Items.ProductID = Products.ProductID
            ";

    let items: Vec<ItemProduct> = sqlx::query_as::<_, ItemProduct>(query).fetch_all(&app.lock().unwrap().pool).await?;

    Ok(items)
}

fn get_data(app: &mut App, items: &Option<&[ItemProduct]>) -> Result<Table<'static>> {
    if app.current_screen == CurrentScreen::Main {
        let mut rows = Vec::new();
        if let Some(page) = items {
            rows = page.iter().map(|i| {
                Row::new(vec![
                    i.item_id.to_string(),
                    i.name_of_product.clone(),
                    i.type_of_product.clone(),
                    i.size.clone(),
                    i.level_of_use.to_string(),
                ])
            }).collect();
        }

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

        Ok(table)
    } else {
        Ok(Table::default())
    }
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<bool> {
    let app_clone = Arc::new(Mutex::new(app.clone()));
    let data = fetch_data(&app_clone).await.unwrap_or_else(|_| {
        eprintln!("Error fetching data");
        Vec::new()
    });

    // Store the iterator and its values in variables
    let data_iterator: Vec<&[ItemProduct]> = data.chunks(15).collect();
    app.max_page = data_iterator.len();
    let mut data_to_display = data_iterator[0];

    loop {
        let table = match get_data(app, &Some(data_to_display)) {
            Ok(table) => Some(table),
            Err(_) => None
        };

        terminal.draw(|f| {
            ui(f, app, table).expect("Error rendering the UI");
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Settings;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exit;
                    }
                    KeyCode::Char('a') => {
                        info!("Moving to prev page!");
                        if app.current_page > 0 {
                            app.current_page = (app.current_page - 1) % app.max_page;
                            info!("new page: {}", app.current_page);
                            data_to_display = data_iterator[app.current_page];
                        } else {
                            app.current_page = app.max_page;
                            data_to_display = data_iterator[app.current_page];
                        }
                    }
                    KeyCode::Char('d') => {
                        info!("moving to next page");
                        app.current_page = (app.current_page + 1) % app.max_page;
                        info!("new page: {}", app.current_page);
                        data_to_display = data_iterator[app.current_page];
                    }
                    _ => {}
                },
                CurrentScreen::Exit => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Settings => match key.code {
                    KeyCode::Char('c') => app.current_screen = CurrentScreen::Config,
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exit,
                    _ => {}
                },
                _ => match key.code {
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exit,
                    _ => {}
                }
            }
        }
    }
}