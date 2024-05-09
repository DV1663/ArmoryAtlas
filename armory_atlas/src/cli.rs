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
    pub subcommands: Option<CommandType>,
}

#[derive(Subcommand)]
pub enum CommandType {
    Config(AppConfig),
    Generate(GenerateArgs),
    Manage(ManageArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ManageArgs {
    #[arg(short, long)]
    pub drop_tables: bool,
    #[arg(short, long)]
    pub create_tables: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum GenerateSubCommands {
    Products,
    Items(ItemsArgs),
    Users(UsersArgs),
    Loans(LoansArgs),
}

#[derive(Args, Debug, Clone)]
pub struct UsersArgs {
    #[arg(default_value = "100")]
    pub num_users: usize,
}

#[derive(Args, Debug, Clone)]
pub struct LoansArgs {
    #[arg(default_value = "100")]
    pub num_loans: usize,
}

#[derive(Args, Debug, Clone)]
pub struct ItemsArgs {
    #[arg(default_value = "100")]
    pub num_items: usize,
}

#[derive(Args, Debug, Clone)]
pub struct GenerateArgs {
    #[command(subcommand)]
    pub subcommands: Option<GenerateSubCommands>,
}
