use std::io::Read;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub mod config;
pub mod cli;
pub mod password_handler;

pub const CONFIG_FILE: &str = ".config/armoryatlas/config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    #[serde(rename = "ProductID")]
    pub product_id: String,
    #[serde(rename = "NameOfProduct")]
    pub product_name: String,
    #[serde(rename = "Type")]
    pub product_type: String
}

pub fn generate_products() -> Result<Vec<Products>> {
    let mut file = std::fs::File::open("products.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let products: Vec<Products> = serde_json::from_str(&contents)?;
    Ok(products)
}

pub fn configure() -> Result<()> {
    Ok(())
}
