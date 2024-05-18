use std::fs;
use std::fs::File;

use crate::cli::{Command, CommandType, GenerateArgs, GenerateSubCommands, GetSubCommands, ReturnSubCommands};
use crate::items::{insert_items, Item};
use crate::products::insert_products;
use anyhow::Result;
use chrono::Local;
use env_logger::{Builder, Env};
use pyo3::prelude::*;
use std::io::Write;
use clap::Parser;
use log::{debug, info};
use prettytable::Table;

use regex::Regex;


pub mod cli;
pub mod config;
pub mod db_handler;
pub mod items;
pub mod leandings;
pub mod password_handler;
pub mod products;
#[cfg(feature = "tui")]
pub mod tui;
pub mod users;

pub const CONFIG_DIR: &str = ".config/armoryatlas";
pub const CONFIG_FILE: &str = "config.toml";
pub const PRODUCTS_FILE: &str = "products.json";
pub const DEFAULT_PRODUCTS: &str = include_str!("../../default-products.json");
pub const DEFAULT_CONFIG: &str = include_str!("../../default-config.toml");
pub const DATABASE_HANDLER: &str = include_str!("../ArmoryAtlasDBHandler.py");

use crate::config::{get_config, write_config};
use crate::db_handler::{DBHandler, DetailedItem, DetailedItems};
use crate::db_handler::in_stock_size::InStockSizes;
use crate::db_handler::loans::DetailedLoans;
use crate::db_handler::users::Users;
use crate::password_handler::get_db_pass;
use crate::users::User;

#[derive(Debug, FromPyObject, Clone)]
#[cfg_attr(feature = "rs-db", derive(sqlx::FromRow))]
pub struct ItemProduct {
    product_id: String,
    product_name: String,
    product_type: String,
    quantity: i64,
    size: String,
}

pub async fn search_items(search_param: &str) -> Result<Vec<DetailedItem>> {
    let items = DBHandler::new()?.search_items(search_param)?;

    Ok(items)
}

pub fn generate_test_data(args: GenerateArgs, db_handler: DBHandler) -> Result<()> {
    match args.subcommands {
        Some(GenerateSubCommands::Products) => insert_products(&db_handler)?,
        
        Some(GenerateSubCommands::Items(sub_args)) => {
            insert_items(&db_handler, sub_args.num_items)?
        }
        
        Some(GenerateSubCommands::Users(sub_args)) => {
            users::insert_users(&db_handler, sub_args.num_users)?
        },
        
        Some(GenerateSubCommands::Loans(sub_args)) => {
            println!("Inserting {} loans", sub_args.num_loans);
            leandings::insert_leandings(&db_handler, sub_args.num_loans)?
        }
        
        _ => {
            println!("No subcommand provided. Generating for all tables with default values...");
            
            match insert_products(&db_handler) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error inserting products: {}\nProducts might already be in the database", e);
                    
                }
            }
            
            insert_items(&db_handler, args.num_to_generate.unwrap())?;
            users::insert_users(&db_handler, args.num_to_generate.unwrap())?;
            leandings::insert_leandings(&db_handler, args.num_to_generate.unwrap())?;
        }
    }

    Ok(())
}

pub fn extract_sql_from_file(file_name: &str) -> Result<Vec<String>> {
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

pub fn extract_sql_from_string(content: &str) -> Result<Vec<String>> {
    let comment_re = Regex::new(r"(--.*$)|(/\*[\s\S]*?\*/)|(#.*$)")?;
    // Remove comments
    let no_comments = comment_re.replace_all(content, "");

    // Split by ';' to separate SQL statements
    let statements: Vec<&str> = no_comments
        .split(';')
        .filter(|s| !s.trim().is_empty()) // Remove empty statements
        .collect();

    let res = statements.iter().map(|query| query.to_string()).collect();

    Ok(res)
}

pub fn setup_logger() -> Result<()> {
    // Get the current timestamp
    let now = Local::now();
    // Format the timestamp as a string in the desired format
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    // Create the log filename with the timestamp
    let log_filename = format!("logs/{}.log", timestamp);
    // Create the log file and directory if needed
    fs::create_dir_all("logs")?;

    let file = File::create(log_filename)?;

    Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} - {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record
                    .file()
                    .unwrap_or(record.module_path().unwrap_or("unknown")),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
    Ok(())
}

#[pyfunction]
pub fn run_cli(args: Option<Vec<String>>) -> Result<()> {
    info!("Starting Armory Atlas...");
    setup_logger()?;

    let cmd = if let Some(args) = args {
        Command::parse_from(args)
    } else {
        Command::parse()
    };
    
    let config = get_config()?;

    let (user, host, database) = (
        cmd.user.unwrap_or(config.get("user")?),
        cmd.host.unwrap_or(config.get("host")?),
        cmd.database.unwrap_or(config.get("database")?),
    );

    let password = get_db_pass(&user, &host)?;

    let db_handler = DBHandler::new()?;

    match cmd.subcommands {
        Some(CommandType::Config(args)) => {
            write_config(&args, &password)?;
        }
        Some(CommandType::Generate(args)) => {
            generate_test_data(args, db_handler)?;
        }
        Some(CommandType::Manage(args)) => {
            if args.drop_all {
                db_handler.drop_all()?;
            }
            if args.create_all {
                db_handler.create_all()?;
            }
        },
        Some(CommandType::Get(args)) => {
            match args.subcommands {
                GetSubCommands::Items(args) => {
                    if args.limit.is_none() {
                        // if limit is not specified we get all
                        let items: DetailedItems = db_handler.get_items()?.into();
                        println!("{}", Table::from(items))
                    } else {
                        let items: DetailedItems = db_handler.get_items()?[..args.limit.unwrap()].to_vec().into();
                        println!("{}", Table::from(items))
                    }
                }
                GetSubCommands::InStock(args) => {
                    let items = db_handler.get_in_stock_size(args.pruduct_id, args.size)?;
                    println!("{}", Table::from(items));
                }
                GetSubCommands::Loans(args) => {
                    if args.limit.is_none() {
                        // if limit is not specified we get all
                        let loans: DetailedLoans = if args.ssn.is_some() {
                            db_handler.user_all_borrowed(args.ssn.unwrap())?.into()
                        } else {
                            db_handler.get_loans()?.into()
                        };
                        println!("{}", Table::from(loans))
                    } else {
                        let loans: DetailedLoans = if args.ssn.is_some() {
                            db_handler.user_all_borrowed(args.ssn.unwrap())?[..args.limit.unwrap()].to_vec().into()
                        } else {
                            db_handler.get_loans()?[..args.limit.unwrap()].to_vec().into()
                        };
                        println!("{}", Table::from(loans))
                    }
                }
                GetSubCommands::Users(args) => {
                    if args.limit.is_none() {
                        let users: Users = db_handler.get_users()?.into();
                        println!("{}", Table::from(users));
                    } else {
                        let users: Users = db_handler.get_users()?[..args.limit.unwrap()].to_vec().into();
                        println!("{}", Table::from(users));
                    }
                }
            }
        }
        Some(CommandType::Return(args)) => {
            match args.subcommands {
                ReturnSubCommands::Item(args) => {
                    db_handler.return_item(args.item_id)?;
                }
            }
        }
        _ => {
            //run_tui(pool).await?;
        }
    };

    Ok(())
}

#[pymodule]
fn armory_atlas_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Item>()?;
    m.add_class::<DBHandler>()?;

    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}