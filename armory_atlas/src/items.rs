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

use std::fmt::Display;
use anyhow::Result;
use prettytable::{row, Row, Table};
use pyo3::{FromPyObject, pymethods};
use rand::Rng;
use rayon::prelude::*;
use crate::db_handler::DBHandler;

use crate::products;

pub const SIZES: [&str; 6] = ["XS", "S", "M", "L", "XL", "XXL"];

#[derive(Debug, FromPyObject)]
#[cfg_attr(feature = "rs-db", derive(sqlx::FromRow))]
#[pyo3::pyclass]
pub struct Item {
    pub item_id: String,
    pub product_id: String,
    pub size: String,
    pub quality: f32,
}

#[derive(Clone)]
pub struct TmpItem {
    pub item_id: String,
    pub product_id: String,
    pub size: String,
    pub quality: f32,
}

impl From<&Item> for TmpItem {
    fn from(item: &Item) -> Self {
        Self {
            item_id: item.item_id.clone(),
            product_id: item.product_id.clone(),
            size: item.size.clone(),
            quality: item.quality,
        }
    }
}

impl From<TmpItem> for Item {
    fn from(tmp_item: TmpItem) -> Self {
        Self {
            item_id: tmp_item.item_id,
            product_id: tmp_item.product_id,
            size: tmp_item.size,
            quality: tmp_item.quality,
        }
    }
}

#[pymethods]
impl Item {
    #[new]
    pub fn new(item_id: String, product_id: String, size: String, quality: f32) -> Self {
        Self {
            item_id,
            product_id,
            size,
            quality,
        }
    }
    
    #[getter(item_id)]
    pub fn get_item_id(&self) -> String {
        self.item_id.clone()
    }
    
    #[getter(product_id)]
    pub fn get_product_id(&self) -> String {
        self.product_id.clone()
    }
    
    #[getter(size)]
    pub fn get_size(&self) -> String {
        self.size.clone()
    }
    
    #[getter(quality)]
    pub fn get_quality(&self) -> f32 {
        self.quality
    }
    
    #[pyo3(name = "__repr__")]
    pub fn repr(&self) -> String {
        self.to_string()
    }
}

impl From<Item> for Row {
    fn from(item: Item) -> Self {
        if item.item_id.is_empty() {
            row!["WILL_BE_GENERATED", item.product_id, item.size, item.quality]
        } else {
            row![item.item_id, item.product_id, item.size, item.quality]
        }
    }
}

pub struct Items(Vec<Item>);

impl From<Vec<Item>> for Items {
    fn from(items: Vec<Item>) -> Self {
        Self(items)
    }
}

impl From<Items> for Table {
    fn from(items: Items) -> Self {
        let mut table = Table::new();
        table.add_row(row!["ItemID", "ProductID", "Size", "Quality"]);
        for item in items.0 {
            table.add_row(item.into());
        }
        table
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.item_id.is_empty() {
            write!(
                f,
                "Item(item_id=\"WILL_BE_GENERATED\", product_id={}, size={}, quality={})",
                self.product_id, self.size, self.quality
            )
        } else {
            write!(
                f,
                "Item(item_id={}, product_id={}, size={}, quality={})",
                self.item_id, self.product_id, self.size, self.quality
            )
        }
    }
}

pub fn generate_items(num_items: usize) -> Result<Vec<Item>> {
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
                let item = Item {
                    item_id: String::new(),
                    product_id: product.product_id.clone(),
                    size: SIZES[next_idx].to_string(),
                    quality: rng.gen_range(0.0..1.0),
                };

                product_items.push(item);

                next_idx = (next_idx + 1) % SIZES.len();
            }

            product_items
        })
        .collect::<Vec<Vec<Item>>>();

    for product_items in items_iter {
        items.extend(product_items);
    }

    Ok(items)
}

pub fn insert_items(db_handler: &DBHandler, num_items: usize) -> Result<()> {
    let items = generate_items(num_items)?;
    
    println!("Inserting these items:");
    let mut table_vec = Vec::new();
    let mut items_vec = Vec::new();
    
    for item in &items {
        let tmp = TmpItem::from(item);
        let item = Item::from(tmp.clone());
        let item_new = Item::from(tmp);
        table_vec.push(item_new);
        items_vec.push(item);
    }

    let table = Table::from(Items::from(table_vec)).to_string();
    
    println!("{}", table);
    
    for item in items {
        db_handler.insert_item(item)?;
    }

    Ok(())
}
