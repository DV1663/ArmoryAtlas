use crate::config::get_config;
use anyhow::Result;
use sqlx_mysql::MySqlPool;

pub enum CurrentScreen {
    Main,
    Settings,
    Config,
    Exit
}

pub enum CurrentlyEditing {
    Config,
    Search
}

pub struct App {
    pub pool: MySqlPool,
    pub user: String,
    pub host: String,
    pub database: String,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub current_page: usize
}

impl App {
    pub fn new(pool: MySqlPool) -> Result<Self> {
        let config = get_config()?;
        let (user, host, database) = (config.get("user")?, config.get("host")?, config.get("database")?);
        Ok(Self {
            pool,
            user,
            host,
            database,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            current_page: 0
        })
    }

    pub fn next_page(&mut self) -> Result<()> {
        self.current_page += 1;
        Ok(())
    }
}