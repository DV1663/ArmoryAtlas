use anyhow::Result;
use clap::Parser;
use sqlx_mysql::MySqlPool;

use armory_atlas_lib::cli::{Command, CommandType};
use armory_atlas_lib::config::{get_config, write_config};
use armory_atlas_lib::{extract_sql, generate_test_data};

use armory_atlas_lib::password_handler::get_db_pass;

#[tokio::main]
async fn main() -> Result<()> {
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
            write_config(&args)?;
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
            eprintln!("Unknown command passed!")
        }
    };

    Ok(())
}
