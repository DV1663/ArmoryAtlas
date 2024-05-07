use crate::{DATABASE_HANDLER, ItemProduct};
use pyo3::prelude::*;

pub struct DBHandler {
    pool: PyObject,
}

impl DBHandler {
    pub fn new() -> anyhow::Result<Self> {
        let pool = DBHandler::get_db_handler_obj()?;
        Ok(Self { pool })
    }

    pub fn get_items(&self) -> anyhow::Result<Vec<ItemProduct>> {
        Python::with_gil(|py| {
            let items = self.pool.call_method0(py, "get_items")?;
            let items: Vec<ItemProduct> = items.extract(py)?;
            Ok(items)
        })
    }

    pub fn get_db_handler_obj() -> anyhow::Result<PyObject> {
        Python::with_gil(|py| {
            let module = PyModule::from_code(
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
}
