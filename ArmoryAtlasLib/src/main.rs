use anyhow::Result;
use clap::Parser;
use sqlx_mysql::MySqlPool;
use armory_atlas_lib::products::insert_products;
use armory_atlas_lib::cli::{Command, CommandType};
use armory_atlas_lib::config::{get_config, write_config};
use armory_atlas_lib::items::insert_items;
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

    match cmd.subcommands {
        Some(CommandType::Config(args)) => {
            write_config(&args)?;
        }
        _ => {}
    };

    let password = get_db_pass(&user, &host)?;
    
    let pool = MySqlPool::connect(format!("mysql://{user}:{password}@{host}/{database}").as_str()).await?;
    
    //println!("Inserting products");
    //insert_products(&pool).await?;
    println!("Inserting Items");
    insert_items(&pool, 10).await?;

    Ok(())
}