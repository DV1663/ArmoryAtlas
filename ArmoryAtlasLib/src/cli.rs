use crate::config::AppConfig;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    #[arg(short, long)]
    pub user: Option<String>,
    #[arg(short = 'H', long)]
    pub host: Option<String>,
    #[arg(short, long)]
    pub database: Option<String>,
    #[command(subcommand)]
    pub subcommands: CommandType,
}

#[derive(Subcommand)]
pub enum CommandType {
    Config(AppConfig),
}

#[derive(Args)]
pub struct ConfigCommandArgs {
    #[arg(short, long, default_value = "root")]
    user: String,
    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,
    #[arg(short, help = "Store the password in the keyring")]
    password: bool,
}

