#![feature(async_closure)]

use std::fs;

use crate::cli::{GenerateArgs, GenerateSubCommands};
use crate::items::insert_items;
use crate::products::insert_products;
use anyhow::Result;
use regex::Regex;
use sqlx_mysql::MySqlPool;

pub mod cli;
pub mod config;
pub mod items;
pub mod password_handler;
pub mod products;

pub const CONFIG_FILE: &str = ".config/armoryatlas/config.toml";

pub async fn generate_test_data(args: GenerateArgs, pool: &MySqlPool) -> Result<()> {
    match args.subcommands {
        Some(GenerateSubCommands::Products) => insert_products(pool).await?,
        Some(GenerateSubCommands::Items(sub_args)) => {
            insert_items(pool, sub_args.num_items).await?
        }
        _ => {}
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
