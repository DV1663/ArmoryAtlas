use anyhow::Result;
use clap::Parser;
use sqlx_mysql::MySqlPool;

use armory_atlas_lib::cli::{Command, CommandType};
use armory_atlas_lib::config::{get_config, write_config};
use armory_atlas_lib::{extract_sql, generate_test_data};

use armory_atlas_lib::password_handler::get_db_pass;
use armory_atlas_lib::tui::run_tui;

use chrono::Local;
use env_logger::{Builder, Env};
use std::fs::File;
use std::io::Write;

pub fn setup_logger() -> Result<()> {
    // Get the current timestamp
    let now = Local::now();
    // Format the timestamp as a string in the desired format
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    // Create the log filename with the timestamp
    let log_filename = format!("logs/{}.log", timestamp);
    // Create the log file and directory if needed
    std::fs::create_dir_all("logs")?;

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

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;

    let cmd = Command::parse();
    let config = get_config()?;

    let (user, host, database) = (
        cmd.user.unwrap_or(config.get("user")?),
        cmd.host.unwrap_or(config.get("host")?),
        cmd.database.unwrap_or(config.get("database")?),
    );

    let password = get_db_pass(&user, &host)?;
    match cmd.subcommands {
        Some(CommandType::Config(args)) => {
            write_config(&args, &password)?;
        }
        Some(CommandType::Generate(args)) => {
            let pool =
                MySqlPool::connect(format!("mysql://{user}:{password}@{host}/{database}").as_str())
                    .await?;
            generate_test_data(args, &pool).await?;
        }
        Some(CommandType::Manage(args)) => {
            let pool =
                MySqlPool::connect(format!("mysql://{user}:{password}@{host}/{database}").as_str())
                    .await?;
            if args.drop_tables {
                let queries = extract_sql("SQL/Drop.sql")?;

                for query in queries {
                    sqlx::query(&query).execute(&pool).await?;
                }
            }

            if args.create_tables {
                let queries = extract_sql("SQL/Tables.sql")?;

                for query in queries {
                    sqlx::query(&query).execute(&pool).await?;
                }
            }
        }
        _ => {
            let pool =
                MySqlPool::connect(format!("mysql://{user}:{password}@{host}/{database}").as_str())
                    .await?;
            
            run_tui(pool).await?;
        }
    };

    Ok(())
}
