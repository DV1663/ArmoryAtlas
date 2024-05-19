use std::ops::Index;

use prettytable::{Row, row, Table};
use pyo3::prelude::*;
use rayon::prelude::*;

use crate::{DATABASE_HANDLER, ItemProduct};
use crate::db_handler::in_stock_size::{InStockSize, InStockSizes};
use crate::db_handler::loans::{DetailedLoan, PyDetailedLoan};
use crate::db_handler::users::PyUser;
use crate::items::Item;
use crate::leandings::Loans;
use crate::products::Product;
use crate::users::User;

pub mod in_stock_size;
pub mod loans;
pub mod users;

/// The main struct for the database handler
///
/// Currently, it's built with a python implementation used inside the rust code but will later be 
/// replaced with a pure rust implementation.
/// 
/// # Example
/// 
/// ``` no_run
/// # use armory_atlas_lib::db_handler::DBHandler;
/// 
/// let db_handler = DBHandler::new().unwrap();
/// ```
/// 
#[derive(Clone)]
#[pyclass]
pub struct DBHandler {
    pool: PyObject,
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct DetailedItem {
    pub product_id: String,
    pub product_name: String,
    pub product_type: String,
    pub quantity: i64,
    pub size: String,
}

#[pymethods]
impl DetailedItem {
    #[pyo3(name = "__repr__")]
    pub fn repr(&self) -> String {
        format!("{:?}", self)
    }
    
    #[pyo3(name = "__str__")]
    pub fn str(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<ItemProduct> for DetailedItem {
    fn from(item_product: ItemProduct) -> Self {
        Self {
            product_id: item_product.product_id.clone(),
            product_name: item_product.product_name.clone(),
            product_type: item_product.product_type.clone(),
            quantity: item_product.quantity,
            size: item_product.size.clone(),
        }
    }
}

impl From<DetailedItem> for ItemProduct {
    fn from(detailed_item: DetailedItem) -> Self {
        Self {
            product_id: detailed_item.product_id.clone(),
            product_name: detailed_item.product_name.clone(),
            product_type: detailed_item.product_type.clone(),
            quantity: detailed_item.quantity,
            size: detailed_item.size.clone(),
        }
    }
}

impl From<&DetailedItem> for ItemProduct {
    fn from(detailed_item: &DetailedItem) -> Self {
        Self {
            product_id: detailed_item.product_id.clone(),
            product_name: detailed_item.product_name.clone(),
            product_type: detailed_item.product_type.clone(),
            quantity: detailed_item.quantity,
            size: detailed_item.size.clone(),
        }
    }
}

impl From<DetailedItem> for Row {
    fn from(value: DetailedItem) -> Self {
        row![value.product_id, value.product_name, value.product_type, value.quantity, value.size]
    }
}

impl From<&DetailedItem> for Row {
    fn from(value: &DetailedItem) -> Self {
        row![value.product_id, value.product_name, value.product_type, value.quantity, value.size]
    }
}

#[derive(Debug)]
#[pyclass]
pub struct DetailedItems(Vec<DetailedItem>);

#[pymethods]
impl DetailedItems {
    #[getter(items)]
    fn get_items(&self) -> Vec<DetailedItem> {
        self.0.clone()
    }
    #[pyo3(name = "__repr__")]
    pub fn repr(&self) -> String {
        format!("{:?}", self)
    }
    
    #[pyo3(name = "__str__")]
    pub fn str(&self) -> String {
        format!("{:?}", self)
    }
}

impl Index<usize> for DetailedItems {
    type Output = DetailedItem;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<Vec<DetailedItem>> for DetailedItems {
    fn from(items: Vec<DetailedItem>) -> Self {
        Self(items)
    }
}

impl From<DetailedItems> for Vec<DetailedItem> {
    fn from(detailed_items: DetailedItems) -> Self {
        detailed_items.0
    }
}

impl From<DetailedItems> for Table {
    fn from(items: DetailedItems) -> Self {
        let mut table = Table::new();
        table.add_row(row!["Product ID", "Product Name", "Product Type", "Quantity", "Size"]);
        for item in items.0 {
            table.add_row((&item).into());
        }
        table
    }
}

#[pymethods]
impl DBHandler {
    #[new]
    pub fn new() -> anyhow::Result<Self> {
        pyo3::prepare_freethreaded_python();
        let pool = DBHandler::get_db_handler_obj()?;
        Ok(Self { pool })
    }

    pub fn get_items(&self) -> anyhow::Result<Vec<DetailedItem>> {
        Python::with_gil(|py| {
            let items = self.pool.call_method0(py, "get_items")?;
            let items: Vec<ItemProduct> = items.extract(py)?;
            let items: Vec<DetailedItem> = items
                .into_par_iter()
                .map(DetailedItem::from)
                .collect();
            Ok(items)
            
        })
    }

    pub fn get_in_stock_size(&self, product_id: String, size: String) -> anyhow::Result<InStockSizes> {
        Python::with_gil(|py| {
            let items = self.pool.call_method1(py, "get_in_stock_size", (product_id, size))?;
            let items: Vec<InStockSize> = items.extract(py)?;

            Ok(items.into())
        })
    }

    pub fn get_loans(&self) -> anyhow::Result<Vec<DetailedLoan>> {
        Python::with_gil(|py| {
            let loans = self.pool.call_method0(py, "get_loans")?;
            let loans: Vec<PyDetailedLoan> = loans.extract(py)?;
            let loans: Vec<DetailedLoan> = loans
                .into_par_iter()
                .map(DetailedLoan::from)
                .collect();
            Ok(loans)
        })
    }

    #[staticmethod]
    pub fn get_db_handler_obj() -> anyhow::Result<PyObject> {
        Python::with_gil(|py| {
            let module = PyModule::from_code_bound(
                py,
                DATABASE_HANDLER,
                "ArmoryAtlasDBHandler.py",
                "ArmoryAtlasDBHandler",
            )?;
            let db_handler = module.getattr("DBHandler")?;
            let db = db_handler.call0()?.to_object(py);
            Ok(db)
        })
    }

    pub fn get_rand_item(&self) -> anyhow::Result<Item> {
        Python::with_gil(|py| {
            let items = self.pool.call_method0(py, "get_rand_item")?;
            let item: Item = items.extract(py)?;
            Ok(item)
        })
    }

    pub fn get_rand_user(&self) -> anyhow::Result<User> {
        Python::with_gil(|py| {
            let users = self.pool.call_method0(py, "get_rand_user")?;
            let user: PyUser = users.extract(py)?;
            Ok(user.into())
        })
    }
    
    pub fn insert_product(&self, product: Product) -> anyhow::Result<()> {
        Python::with_gil(|py| {
            self.pool.call_method1(py, "insert_product", (product,))?;
            Ok(())
        })
    }
    
    pub fn insert_item(&self, item: Item) -> anyhow::Result<()> {
        Python::with_gil(|py| {
            self.pool.call_method1(py, "insert_item", (item,))?;
            Ok(())
        })
    }
    
    pub fn insert_user(&self, user: User) -> anyhow::Result<()> {
        Python::with_gil(|py| {
            self.pool.call_method1(py, "insert_user", (user,))?;
            Ok(())
        })
    }
    
    pub fn insert_loan(&self, loan: Loans) -> anyhow::Result<()> {
        Python::with_gil(|py| {
            self.pool.call_method1(py, "insert_loan", (loan,))?;
            Ok(())
        })
    }
    
    pub fn search_items(&self, query: &str) -> anyhow::Result<Vec<DetailedItem>> {
        Python::with_gil(|py| {
            let items = self.pool.call_method1(py, "search_items", (query,))?;
            let items: Vec<ItemProduct> = items.extract(py)?;
            let items: Vec<DetailedItem> = items
                .into_par_iter()
                .map(DetailedItem::from)
                .collect();
            Ok(items)
        })
    }

    pub fn drop_all(&self) -> anyhow::Result<()> {
        Python::with_gil(|py| {
            self.pool.call_method0(py, "drop_all")?;
            Ok(())
        })
    }

    pub fn create_all(&self) -> anyhow::Result<()> {
        Python::with_gil(|py| {
            self.pool.call_method0(py, "create_all")?;
            Ok(())
        })
    }

    pub fn get_users(&self) -> anyhow::Result<Vec<User>> {
        Python::with_gil(|py| {
            let users = self.pool.call_method0(py, "get_users")?;
            let users: Vec<PyUser> = users.extract(py)?;
            let users = users.into_par_iter().map(User::from).collect();
            Ok(users)
        })
    }
    
    pub fn return_item(&self, item_id: String) -> anyhow::Result<()> {
        Python::with_gil(|py| {
            self.pool.call_method1(py, "return_item", (item_id,))?;
            Ok(())
        })
    }
    
    pub fn user_all_borrowed(&self, ssn: String) -> anyhow::Result<Vec<DetailedLoan>> {
        Python::with_gil(|py| {
            let loans = self.pool.call_method1(py, "user_all_borrowed", (ssn,))?;
            let loans: Vec<PyDetailedLoan> = loans.extract(py)?;
            let loans: Vec<DetailedLoan> = loans
                .into_par_iter()
                .map(DetailedLoan::from)
                .collect();
            Ok(loans)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_db_handler_obj() {
        let db_handler = DBHandler::get_db_handler_obj();
        match db_handler {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[test]
    fn test_get_items() {
        let db_handler = DBHandler::new();
        assert!(db_handler.is_ok());

        let db_handler = db_handler.unwrap();
        let items = db_handler.get_items();
        assert!(items.is_ok());
    }
    
    #[test]
    fn test_get_rand_item() {
        let db_handler = DBHandler::new();
        assert!(db_handler.is_ok());

        let db_handler = db_handler.unwrap();
        let item = db_handler.get_rand_item();
        
        assert!(item.is_ok());

        let item = item.unwrap();
        println!("{:?}", item);
    }
    
    #[test]
    fn test_get_rand_user() {
        let db_handler = DBHandler::new();
        assert!(db_handler.is_ok());

        let db_handler = db_handler.unwrap();
        let user = db_handler.get_rand_user();
        assert!(user.is_ok());

        let user = user.unwrap();
        println!("{:?}", user);
    }
}
