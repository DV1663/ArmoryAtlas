use std::fs;

use crate::cli::{GenerateArgs, GenerateSubCommands};
use crate::items::insert_items;
use crate::products::insert_products;
use anyhow::Result;
use pyo3::FromPyObject;

use regex::Regex;
use sqlx_mysql::MySqlPool;

pub mod cli;
pub mod config;
pub mod db_handler;
pub mod items;
pub mod leandings;
pub mod password_handler;
pub mod products;
pub mod tui;
pub mod users;

pub const CONFIG_DIR: &str = ".config/armoryatlas";
pub const CONFIG_FILE: &str = "config.toml";
pub const PRODUCTS_FILE: &str = "products.json";
pub const DEFAULT_PRODUCTS: &str = include_str!("../../default-products.json");
pub const DEFAULT_CONFIG: &str = include_str!("../../default-config.toml");
pub const DATABASE_HANDLER: &str = include_str!("../ArmoryAtlasDBHandler.py");

use sqlx::FromRow;
use crate::db_handler::DBHandler;

#[derive(Debug, FromRow, Clone, FromPyObject)]
pub struct ItemProduct {
    product_id: String,
    product_name: String,
    product_type: String,
    quantity: i64,
    size: String,
}

pub async fn search_items(search_param: &str) -> Result<Vec<ItemProduct>> {
    let items = DBHandler::new()?.search_items(search_param)?;

    Ok(items)
}

pub async fn generate_test_data(args: GenerateArgs, db_handler: DBHandler) -> Result<()> {
    match args.subcommands {
        Some(GenerateSubCommands::Products) => insert_products(&db_handler).await?,
        
        Some(GenerateSubCommands::Items(sub_args)) => {
            insert_items(&db_handler, sub_args.num_items).await?
        }
        
        Some(GenerateSubCommands::Users(sub_args)) => {
            users::insert_users(&db_handler, sub_args.num_users).await?
        },
        
        Some(GenerateSubCommands::Loans(sub_args)) => {
            println!("Inserting {} loans", sub_args.num_loans);
            leandings::insert_leandings(&db_handler, sub_args.num_loans).await?
        }
        
        _ => {
            println!("No subcommand provided. Generating for all tables with default values...");
            
            match insert_products(&db_handler).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error inserting products: {}\nProducts might already be in the database", e);
                }
            }
            
            insert_items(&db_handler, 10).await?;
            users::insert_users(&db_handler, 10).await?;
            leandings::insert_leandings(&db_handler, 10).await?;
        }
    }

    Ok(())
}

pub fn extract_sql(file_name: &str) -> Result<Vec<String>> {
    // Load the file content
    let content = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    // Regex to match single-line and block comments
    let comment_re = Regex::new(r"(--.*$)|(/\*[\s\S]*?\*/)|(#.*$)")?;
    // Remove comments
    let no_comments = comment_re.replace_all(&content, "");

    // Split by ';' to separate SQL statements
    let statements: Vec<&str> = no_comments
        .split(';')
        .filter(|s| !s.trim().is_empty()) // Remove empty statements
        .collect();

    let res = statements.iter().map(|query| query.to_string()).collect();

    Ok(res)
}
