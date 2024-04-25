use config::Config;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use clap::Args;
use crate::CONFIG_FILE;

#[derive(Args, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[arg(short, long, default_value = "root")]
    pub user: String,
    #[arg(short = 'H', long, default_value = "localhost")]
    pub host: String,
    #[arg(short, long, default_value = "ArmoryAtlas")]
    pub database: String
}

pub fn get_config() -> anyhow::Result<Config> {
    let path = PathBuf::new().join(env!("HOME")).join(CONFIG_FILE);

    if !path.exists() {
        println!("Config file does not exist. Creating it...");
        create_config_file(&path)?;
    }

    let settings = Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .set_default("user", "root")?
        .set_default("host", "localhost")?
        .set_default("database", "ArmoryAtlas")?
        .build()?;
    Ok(settings)
}

pub fn write_config(app_config: &AppConfig) -> anyhow::Result<()> {
    let path = PathBuf::new().join(env!("HOME")).join(CONFIG_FILE);

    if !path.exists() {
        println!("Config file does not exist. Creating it...");
        create_config_file(&path)?;
    }

    let mut file = std::fs::OpenOptions::new().write(true).open(path)?;

    file.write_all(toml::to_string(app_config)?.as_bytes())?;
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
