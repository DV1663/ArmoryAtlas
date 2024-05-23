use anyhow::Result;

use sqlx_mysql::MySqlPool;

use crate::config::get_config;
use crate::python_db_handler::DBHandlerPy;

#[derive(Clone, Eq, PartialEq)]
pub enum CurrentScreen {
    Main,
    Settings,
    Config,
    Exit,
}

#[derive(Clone)]
pub enum CurrentlyEditing {
    Config,
    Search,
}

#[derive(Clone)]
pub struct App {
    pub db_handler: DBHandlerPy,
    pub pool: MySqlPool,
    pub user: String,
    pub host: String,
    pub database: String,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub current_page: usize,
    pub items_per_page: usize,
    pub max_page: usize,
}

impl App {
    pub fn new(pool: MySqlPool) -> Result<Self> {
        let config = get_config()?;
        let (user, host, database) = (
            config.get("user")?,
            config.get("host")?,
            config.get("database")?,
        );

        let db_handler = DBHandlerPy::new()?;
        Ok(Self {
            db_handler,
            pool,
            user,
            host,
            database,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            current_page: 0,
            items_per_page: 15,
            max_page: 0,
        })
    }

    pub fn next_page(&mut self) -> Result<()> {
        self.current_page += 1;
        Ok(())
    }
}
