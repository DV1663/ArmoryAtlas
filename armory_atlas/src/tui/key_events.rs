use crate::tui::app::{App, CurrentScreen};
use crate::ItemProduct;
use crossterm::event::{KeyCode, KeyEvent};
use log::info;

pub fn screen_key_events(
    app: &mut App,
    key: KeyEvent,
    data_iterator: &[Vec<ItemProduct>],
    mut data_to_display: Vec<ItemProduct>,
) -> (bool, Vec<ItemProduct>) {
    match app.current_screen {
        CurrentScreen::Main => match key.code {
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::Settings;
                (false, data_to_display)
            }
            KeyCode::Left => {
                info!("Moving to prev page!");
                if app.current_page > 0 {
                    app.current_page = (app.current_page - 1) % app.max_page;
                    info!("new page: {}", app.current_page);
                    data_to_display.clone_from(&data_iterator[app.current_page]);
                } else {
                    app.current_page = app.max_page - 1;
                    data_to_display.clone_from(&data_iterator[app.current_page]);
                }
                (false, data_to_display)
            }
            KeyCode::Right => {
                info!("moving to next page");
                app.current_page = (app.current_page + 1) % app.max_page;
                info!("new page: {}", app.current_page);
                data_to_display.clone_from(&data_iterator[app.current_page]);
                (false, data_to_display)
            }
            _ => (false, data_to_display),
        },
        CurrentScreen::Exit => match key.code {
            KeyCode::Char('y') => (true, data_to_display),
            KeyCode::Char('n') => {
                app.current_screen = CurrentScreen::Main;
                (false, data_to_display)
            }
            _ => (false, data_to_display),
        },
        CurrentScreen::Settings => match key.code {
            KeyCode::Char('c') => {
                app.current_screen = CurrentScreen::Config;
                (false, data_to_display)
            }
            KeyCode::Char('q') => {
                app.current_screen = CurrentScreen::Exit;
                (false, data_to_display)
            }
            _ => (false, data_to_display),
        },
        _ => match key.code {
            KeyCode::Char('q') => {
                app.current_screen = CurrentScreen::Exit;
                (false, data_to_display)
            }
            _ => (false, data_to_display),
        },
    }
}
