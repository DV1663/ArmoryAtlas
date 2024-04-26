mod ui;
mod app;

use crate::tui::ui::ui;
use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::Terminal;
use sqlx_mysql::MySqlPool;
use std::io;
use crate::tui::app::{App, CurrentScreen};

pub fn run_tui(pool: MySqlPool) -> Result<()> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture);

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(pool)?;
    let res = run_app(&mut terminal, &mut app);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<bool> {
    loop {
        terminal.draw(async move |f| { ui(f, app).await.expect("TODO: panic message"); })?;

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
                    },
                    KeyCode::Left => {
                        if app.current_page > 0 {
                            app.current_page -= 1
                        }
                    },
                    KeyCode::Right => {
                        app.current_page += 1
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
                _ => {}
            }
        }
    }
}