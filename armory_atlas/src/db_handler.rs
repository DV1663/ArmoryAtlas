use crate::items::Item;
use crate::users::Users;
use crate::{ItemProduct, DATABASE_HANDLER};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use rayon::prelude::*;
use crate::leandings::Loans;
use crate::products::Product;

#[derive(Clone)]
#[pyclass]
pub struct DBHandler {
    pool: PyObject,
}

#[pyclass]
pub struct DetailedItem {
    pub product_id: String,
    pub product_name: String,
    pub product_type: String,
    pub quantity: i64,
    pub size: String,
}

impl From<ItemProduct> for DetailedItem {
    fn from(item_product: ItemProduct) -> Self {
        Self {
            product_id: item_product.product_id.clone(),
            product_name: item_product.product_name.clone(),
            product_type: item_product.product_type.clone(),
            quantity: item_product.quantity.clone(),
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

#[pymethods]
impl DBHandler {
    #[new]
    pub fn new() -> anyhow::Result<Self> {
        let pool = DBHandler::get_db_handler_obj()?;
        Ok(Self { pool })
    }

    pub fn get_items(&self) -> anyhow::Result<Vec<DetailedItem>> {
        Python::with_gil(|py| {
            let items = self.pool.call_method0(py, "get_items")?;
            let items: Vec<ItemProduct> = items.extract(py)?;
            let items: Vec<DetailedItem> = items
                .into_par_iter()
                .map(|item| DetailedItem::from(item))
                .collect();
            Ok(items)
            
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

    pub fn get_rand_user(&self) -> anyhow::Result<Users> {
        Python::with_gil(|py| {
            let users = self.pool.call_method0(py, "get_rand_user")?;
            let user: Users = users.extract(py)?;
            Ok(user)
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
    
    pub fn insert_user(&self, user: Users) -> anyhow::Result<()> {
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
                .map(|item| DetailedItem::from(item))
                .collect();
            Ok(items)
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
