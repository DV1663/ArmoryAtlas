#[cfg(feature = "python-db")]
mod python_impl;

#[cfg(feature = "python-db")]
use crate::python_db_handler::DBHandlerPy as DBHandler;
use crate::{CONFIG_DIR, DEFAULT_PRODUCTS, PRODUCTS_FILE};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::cli::InsertProductArgs;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "python-db", pyo3::pyclass)]
pub struct Product {
    #[serde(rename = "ProductID")]
    pub product_id: String,
    #[serde(rename = "NameOfProduct")]
    pub product_name: String,
    #[serde(rename = "Type")]
    pub product_type: String,
}

impl Product {
    pub fn new(product_id: String, product_name: String, product_type: String) -> Self {
        Self {
            product_id,
            product_name,
            product_type,
        }
    }
}

impl From<InsertProductArgs> for Product {
    fn from(args: InsertProductArgs) -> Self {
        Self {
            product_id: args.product_id,
            product_name: args.product_name,
            product_type: args.product_type,
        }
    }
}

pub fn get_products() -> Result<Vec<Product>> {
    let products_file_path = format!("{CONFIG_DIR}/{PRODUCTS_FILE}");

    #[cfg(not(target_os = "windows"))]
    let path = PathBuf::new().join(env!("HOME")).join(products_file_path);

    #[cfg(target_os = "windows")]
    let path = PathBuf::new()
        .join(env!("USERPROFILE"))
        .join(products_file_path);

    if !path.exists() {
        println!("Products file not found, creating it...");
        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut file = std::fs::File::create(&path)?;
        file.write_all(DEFAULT_PRODUCTS.as_ref())?;
    }

    let mut file = std::fs::File::open(&path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let products: Vec<Product> = serde_json::from_str(&contents)?;

    Ok(products)
}

pub fn insert_products(db_handler: &DBHandler) -> Result<()> {
    let products: Vec<Product> = get_products()?;
    //dbg!(&products);
    for product in products {
        db_handler.insert_product(product)?
    }

    Ok(())
}
