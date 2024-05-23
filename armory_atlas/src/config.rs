use std::io::Write;
use std::path::PathBuf;

use crate::{CONFIG_DIR, CONFIG_FILE};
use clap::Args;
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Args, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[arg(short, long, default_value = "root", help = "The user to connect to the database with.")]
    pub user: String,
    #[arg(short = 'H', long, default_value = "localhost", help = "The host to connect to the database with.")]
    pub host: String,
    #[arg(short, long, default_value = "ArmoryAtlas", help = "The name of the database to connect to.")]
    pub database: String,
}

pub fn get_config() -> anyhow::Result<Config> {
    #[cfg(not(target_os = "windows"))]
    let path = PathBuf::new()
        .join(env!("HOME"))
        .join(format!("{CONFIG_DIR}/{CONFIG_FILE}"));

    #[cfg(target_os = "windows")]
    let path = PathBuf::new()
        .join(env!("USERPROFILE"))
        .join(format!("{CONFIG_DIR}/{CONFIG_FILE}"));

    if !path.exists() {
        println!("Config file does not exist. Creating it...");
        create_config_file(&path)?;
    }

    let settings = Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .set_default("user", "root")?
        .set_default("host", "localhost")?
        .set_default("database", "ArmoryAtlas")?
        .set_default("password", "")?
        .build()?;
    Ok(settings)
}

pub fn write_config(app_config: &AppConfig, password: &str) -> anyhow::Result<()> {
    #[cfg(not(target_os = "windows"))]
    let path = PathBuf::new()
        .join(env!("HOME"))
        .join(format!("{CONFIG_DIR}/{CONFIG_FILE}"));

    #[cfg(target_os = "windows")]
    let path = PathBuf::new()
        .join(env!("USERPROFILE"))
        .join(format!("{CONFIG_DIR}/{CONFIG_FILE}"));

    if !path.exists() {
        println!("Config file does not exist. Creating it...");
        create_config_file(&path)?;
    }

    let config = toml::to_string(app_config)?;

    let mut file = std::fs::OpenOptions::new().write(true).open(path)?;

    file.write_all(format!("{}\npassword=\"{}\"", config, password).as_bytes())?;
    Ok(())
}

fn create_config_file(path: &PathBuf) -> anyhow::Result<()> {
    // create parent directory and file if it doesn't exist
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::File::create(path)?;
    }
    Ok(())
}
