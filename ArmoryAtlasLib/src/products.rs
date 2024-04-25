use anyhow::Result;
use std::io::Read;
use serde::{Deserialize, Serialize};
use sqlx_mysql::MySqlPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    #[serde(rename = "ProductID")]
    pub product_id: String,
    #[serde(rename = "NameOfProduct")]
    pub product_name: String,
    #[serde(rename = "Type")]
    pub product_type: String
}

pub fn get_products() -> Result<Vec<Products>> {
    let mut file = std::fs::File::open("products.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let products: Vec<Products> = serde_json::from_str(&contents)?;

    Ok(products)
}

pub async fn insert_products(pool: &MySqlPool) -> Result<()> {
    let products: Vec<Products> = get_products()?;

    // This will be moved out into python at a later date
    {
        println!("Inserting {} products...", products.len());

        for product in &products {
            sqlx::query("INSERT INTO Products (ProductID, NameOfProduct, Type) VALUES (?, ?, ?)")
                .bind(&product.product_id)
                .bind(&product.product_name)
                .bind(&product.product_type)
                .execute(pool)
                .await?;
        }
    }
    
    Ok(())
}