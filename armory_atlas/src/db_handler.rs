use pyo3::prelude::*;
use crate::DATABASE_HANDLER;

pub struct DBHandlerMaster {
    pool: PyObject,
}

impl DBHandlerMaster {
    pub fn new() -> anyhow::Result<Self> {
        let pool = DBHandlerMaster::get_db_handler_obj()?;
        Ok(Self {
            pool,
        })
    }
    
    pub fn get_items(&self) -> anyhow::Result<Vec<(String, String, String, usize, String)>> {
        Python::with_gil(|py| {
            let items = self.pool.call_method0(py, "get_items")?;
            let items: Vec<(String, String, String, usize, String)> = items.extract(py)?;
            Ok(items)
        })
    }

    pub fn get_db_handler_obj() -> anyhow::Result<PyObject> {
        Python::with_gil(|py| {
            let module = PyModule::from_code(py, DATABASE_HANDLER, "ArmoryAtlasDBHandler.py", "ArmoryAtlasDBHandler")?;
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
        let db_handler = DBHandlerMaster::get_db_handler_obj();
        match db_handler { 
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "{:?}", e)
        }
    }
    
    #[test]
    fn test_get_items() {
        let db_handler = DBHandlerMaster::new();
        assert!(db_handler.is_ok());

        let db_handler = db_handler.unwrap();
        let items = db_handler.get_items();
        match items { 
            Ok(items) => {
                assert!(true);
                dbg!(items);
            },
            Err(e) => assert!(false, "{:?}", e)
        }
    }
}