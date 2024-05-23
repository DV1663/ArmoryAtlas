use crate::products::Product;
use pyo3::pymethods;

#[pymethods]
impl Product {
    #[new]
    pub fn py_new(product_id: String, product_name: String, product_type: String) -> Self {
        Self {
            product_id,
            product_name,
            product_type,
        }
    }

    #[getter(product_id)]
    pub fn py_get_product_id(&self) -> String {
        self.product_id.clone()
    }

    #[getter(product_name)]
    pub fn py_get_product_name(&self) -> String {
        self.product_name.clone()
    }

    #[getter(product_type)]
    pub fn py_get_product_type(&self) -> String {
        self.product_type.clone()
    }
}
