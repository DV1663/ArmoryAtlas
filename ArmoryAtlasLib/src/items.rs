/*
INSERT INTO `Items` (`ItemID`, `ProductID`, `Size`, `LevelOfUse`)
VALUES (UUID_TO_BIN(UUID()), 'your_product_id', 'L', 0.5);

CREATE TABLE `Items` (
  `ItemID` binary(16) NOT NULL,
  `ProductID` varchar(50) NOT NULL,
  `Size` varchar(5) DEFAULT NULL,
  `LevelOfUse` float NOT NULL,
  PRIMARY KEY (`ItemID`),
  KEY `FKs` (`ProductID`),
  CONSTRAINT `FKs` FOREIGN KEY (`ProductID`) REFERENCES `Products` (`ProductID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
*/

use anyhow::Result;
use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx_mysql::MySqlPool;

use crate::products;

pub const SIZES: [&str; 6] = ["XS", "S", "M", "L", "XL", "XXL"];

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "ProductID")]
    pub product_id: String,
    #[serde(rename = "Size")]
    pub size: String,
    #[serde(rename = "LevelOfUse")]
    pub level_of_use: f32,
}

pub fn generate_items(num_items: usize) -> Result<Vec<Items>> {
    let products = products::get_products()?;

    println!(
        "Generating {} items for {} diffrent products!",
        num_items,
        products.len()
    );

    let mut items = Vec::new();

    let items_iter = products
        .par_iter()
        .map(|product| {
            let mut product_items = Vec::new();
            let mut rng = rand::thread_rng();

            let mut next_idx = 0;

            for _ in 0..num_items {
                let item = Items {
                    item_id: String::new(),
                    product_id: product.product_id.clone(),
                    size: SIZES[next_idx].to_string(),
                    level_of_use: rng.gen_range(0.0..1.0),
                };

                product_items.push(item);

                next_idx = (next_idx + 1) % SIZES.len();
            }

            product_items
        })
        .collect::<Vec<Vec<Items>>>();

    for product_items in items_iter {
        items.extend(product_items);
    }

    Ok(items)
}

pub async fn insert_items(pool: &MySqlPool, num_items: usize) -> Result<()> {
    let items = generate_items(num_items)?;

    for item in &items {
        sqlx::query("INSERT INTO Items (ItemID, ProductID, Size, LevelOfUse) VALUES (UUID_TO_BIN(UUID()), ?, ?, ?)")
            .bind(&item.product_id)
            .bind(&item.size)
            .bind(item.level_of_use)
            .execute(pool)
            .await?;
    }

    Ok(())
}
