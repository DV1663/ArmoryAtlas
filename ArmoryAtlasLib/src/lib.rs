#![feature(async_closure)]

use std::fs;

use crate::cli::{GenerateArgs, GenerateSubCommands};
use crate::items::insert_items;
use crate::products::insert_products;
use anyhow::Result;
use pyo3::Python;
use pyo3::types::PyModule;

use regex::Regex;
use sqlx_mysql::MySqlPool;
use pyo3::prelude::*;

pub mod cli;
pub mod config;
pub mod items;
pub mod password_handler;
pub mod products;
pub mod tui;
pub mod db_handler;

pub const CONFIG_FILE: &str = ".config/armoryatlas/config.toml";
pub const DATABASE_HANDLER: &str = include_str!("./ArmoryAtlasDBHandler.py");

use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Clone)]
pub struct ItemProductTest {
    item_id: Uuid,
    name_of_product: String,
    type_of_product: String,
    size: String,
    level_of_use: f32,
}

#[derive(Debug, FromRow, Clone)]
pub struct ItemProduct {
    product_id: String,
    product_name: String,
    product_type: String,
    size: String,
    quantity: i64,
}

pub async fn search_items(pool: &MySqlPool, query: &str) -> Result<Vec<ItemProduct>> {
    let query = format!("
        SELECT
            i.ProductID as product_id,
            p.NameOfProduct AS product_name,
            p.Type AS product_type,
            i.Quantity as quantity,
            i.Size AS size
        FROM
            Products p
                JOIN
            (SELECT ProductID, Size, count(*) as Quantity from Items group by ProductID, Size)
                AS
                i ON p.ProductID = i.ProductID
        WHERE
            i.Quantity > 0
            AND
            (
                p.NameOfProduct LIKE '%{query}%' OR
                p.Type LIKE '%{query}%' OR
                i.Size LIKE '%{query}%'
            )
        ORDER BY
            p.NameOfProduct
    ");

    let items: Vec<ItemProduct> = sqlx::query_as::<_, ItemProduct>(query.as_str())
        .fetch_all(pool)
        .await?;
    
    Ok(items)
}

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


