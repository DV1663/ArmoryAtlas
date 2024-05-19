#![allow(unused_imports)]

use std::fs;
use std::fs::File;

use crate::cli::{
    Command, CommandType, GenerateArgs, GenerateSubCommands, GetSubCommands, ReturnSubCommands,
};
use crate::items::{insert_items, Item};
use crate::products::insert_products;
use anyhow::Result;
use chrono::Local;
use clap::Parser;
use env_logger::{Builder, Env};
use log::{debug, info};
use prettytable::Table;
use pyo3::prelude::*;
use std::io::Write;

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
use crate::db_handler::in_stock_size::{InStockSize, InStockSizes};
use crate::db_handler::loans::{DetailedLoan, DetailedLoans};
use crate::db_handler::users::Users;
use crate::db_handler::{DBHandler, DetailedItem, DetailedItems};
use crate::leandings::Loans;
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

/// Search for items in the database
///
/// # Arguments
///
/// * `search_param`: The search parameter to search for.
///
/// # Example
///
/// ```
/// # use armory_atlas_lib::search_items;
///
/// let items = search_items("test");
/// ```
///
///
pub async fn search_items(search_param: &str) -> Result<Vec<DetailedItem>> {
    let items = DBHandler::new()?.search_items(search_param)?;

    Ok(items)
}

/// Generates test data
///
/// This is the main function to generate test data for an Armory Atlas database.  
///
/// # Arguments
///
/// * `args`: The `GenerateArgs` struct containing the subcommand and the number of items to generate.
/// * `db_handler`: The `DBHandler` struct that handles the database operations.
///
/// # Usage
///
/// Its only meant to be used by the `run_cli` function!
///
fn generate_test_data(args: GenerateArgs, db_handler: DBHandler) -> Result<()> {
    match args.subcommands {
        Some(GenerateSubCommands::Products) => insert_products(&db_handler)?,

        Some(GenerateSubCommands::Items(sub_args)) => {
            insert_items(&db_handler, sub_args.num_items)?
        }

        Some(GenerateSubCommands::Users(sub_args)) => {
            users::insert_users(&db_handler, sub_args.num_users)?
        }

        Some(GenerateSubCommands::Loans(sub_args)) => {
            println!("Inserting {} loans", sub_args.num_loans);
            leandings::insert_leandings(&db_handler, sub_args.num_loans)?
        }

        _ => {
            println!("No subcommand provided. Generating for all tables with default values...");

            match insert_products(&db_handler) {
                Ok(_) => {}
                Err(e) => {
                    println!(
                        "Error inserting products: {}\nProducts might already be in the database",
                        e
                    );
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

/// Setup the logger
///  
/// # Example
///
/// ```no_run
/// # use armory_atlas_lib::setup_logger;
/// setup_logger() // This can only be run once!
/// # .unwrap();
/// ```
///
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
/// Executes the command-line interface for the Armory Atlas application.
///
/// This function parses the given command-line arguments and performs actions
/// based on the parsed commands and subcommands. It handles configuration,
/// generation of test data, and database management tasks such as creating
/// and dropping tables.
///
/// # Arguments
///
/// * `args`: An optional vector of strings representing the command-line arguments.
///
/// # Errors
///
/// This function returns an error if any operation such as parsing arguments,
/// accessing configuration, connecting to the database, or executing commands fails.
///
/// # Examples
///
/// ```
/// # use armory_atlas_lib::run_cli;
/// # fn main() -> anyhow::Result<()> {
///     // Running the CLI without any arguments will trigger default command parsing.
///     run_cli(None)?;
///     // Running the CLI with specific arguments.///
///     run_cli(Some(vec!["ArmoryAtlas".to_string(), "get".to_string(), "users".to_string(), "10".to_string()]))?;
/// #   Ok(())
/// # }
/// ```
///
/// # Panics
///
/// This function may panic if unwrapping operations on optional values fail.
pub fn run_cli(args: Option<Vec<String>>) -> Result<()> {
    let cmd = if let Some(args) = args {
        Command::parse_from(args)
    } else {
        Command::parse()
    };

    let config = get_config()?;

    let (user, host, _database) = (
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
        }
        Some(CommandType::Get(args)) => {
            match args.subcommands {
                GetSubCommands::Items(args) => {
                    if args.limit.is_none() {
                        // if limit is not specified we get all
                        let items: DetailedItems = db_handler.get_items()?.into();
                        println!("{}", Table::from(items))
                    } else {
                        let items: DetailedItems = db_handler.get_items()?[..args.limit.unwrap()]
                            .to_vec()
                            .into();
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
                            db_handler.user_all_borrowed(args.ssn.unwrap())?[..args.limit.unwrap()]
                                .to_vec()
                                .into()
                        } else {
                            db_handler.get_loans()?[..args.limit.unwrap()]
                                .to_vec()
                                .into()
                        };
                        println!("{}", Table::from(loans))
                    }
                }
                GetSubCommands::Users(args) => {
                    if args.limit.is_none() {
                        let users: Users = db_handler.get_users()?.into();
                        println!("{}", Table::from(users));
                    } else {
                        let users: Users = db_handler.get_users()?[..args.limit.unwrap()]
                            .to_vec()
                            .into();
                        println!("{}", Table::from(users));
                    }
                }
            }
        }
        Some(CommandType::Return(args)) => match args.subcommands {
            ReturnSubCommands::Item(args) => {
                db_handler.return_item(args.item_id)?;
            }
        },
        _ => {
            //run_tui(pool).await?;
        }
    };

    Ok(())
}

#[allow(deprecated)]
#[pymodule]
fn armory_atlas_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Item>()?;
    m.add_class::<DBHandler>()?;
    m.add_class::<InStockSize>()?;
    m.add_class::<InStockSizes>()?;
    m.add_class::<DetailedItem>()?;
    m.add_class::<DetailedItems>()?;
    m.add_class::<DetailedLoan>()?;
    m.add_class::<DetailedLoans>()?;
    m.add_class::<User>()?;
    m.add_class::<Users>()?;
    m.add_class::<Loans>()?;

    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}
