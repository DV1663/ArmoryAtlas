#[cfg(feature = "python-db")]
mod python_impl;

#[cfg(feature = "python-db")]
use crate::python_db_handler::DBHandlerPy as DBHandler;
use anyhow::Result;
use prettytable::{row, Row, Table};
use rand::Rng;
use rayon::prelude::*;
use std::fmt::Display;
use crate::cli::InsertItemArgs;

use crate::products;

pub const SIZES: [&str; 6] = ["XS", "S", "M", "L", "XL", "XXL"];

#[derive(Debug)]
#[cfg_attr(feature = "python-db", derive(pyo3::FromPyObject))]
#[cfg_attr(feature = "mysql-db", derive(sqlx::FromRow))]
#[cfg_attr(feature = "python-db", pyo3::pyclass)]
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

impl Item {
    pub fn new(item_id: String, product_id: String, size: String, quality: f32) -> Self {
        Self {
            item_id,
            product_id,
            size,
            quality,
        }
    }
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

impl From<Item> for Row {
    fn from(item: Item) -> Self {
        if item.item_id.is_empty() {
            row![
                "WILL_BE_GENERATED",
                item.product_id,
                item.size,
                item.quality
            ]
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

impl From<InsertItemArgs> for Item {
    fn from(insert_item_args: InsertItemArgs) -> Self {
        Self {
            item_id: String::new(),
            product_id: insert_item_args.product_id,
            size: insert_item_args.size,
            quality: insert_item_args.quality,
        }
    }
}