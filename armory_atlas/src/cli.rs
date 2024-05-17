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
    Get(GetArgs),
}

#[derive(Args, Debug, Clone)]
pub struct GetArgs {
    #[command(subcommand)]
    pub subcommands: GetSubCommands,
} 

#[derive(Subcommand, Debug, Clone)]
pub enum GetSubCommands {
    Items(GetItemsArgs),
    InStock(InStockArgs),
    Loans(GetLoansArgs),
}

#[derive(Args, Debug, Clone)]
pub struct GetLoansArgs {
    #[arg(default_value = "10")]
    pub limit: Option<usize>,
}

#[derive(Args, Debug, Clone)]
pub struct GetItemsArgs {
    #[arg(default_value = "10")]
    pub limit: Option<usize>,
}

#[derive(Args, Debug, Clone)]
pub struct InStockArgs {
    #[arg(default_value = "M240001-3708453")]
    pub pruduct_id: String,
    #[arg(default_value = "M")]
    pub size: String,
}

#[derive(Args, Debug, Clone)]
pub struct ManageArgs {
    #[arg(short, long)]
    pub drop_all: bool,
    #[arg(short, long)]
    pub create_all: bool,
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

    #[arg(short, long, default_value = "10", help = "Number of items to generate if no subcommand is provided")]
    pub num_to_generate: Option<usize>,
}
