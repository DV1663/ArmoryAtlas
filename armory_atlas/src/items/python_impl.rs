use crate::items::Item;

#[pyo3::pymethods]
impl Item {
    #[new]
    pub fn py_new(item_id: String, product_id: String, size: String, quality: f32) -> Self {
        Self {
            item_id,
            product_id,
            size,
            quality,
        }
    }

    #[getter(item_id)]
    pub fn py_get_item_id(&self) -> String {
        self.item_id.clone()
    }

    #[getter(product_id)]
    pub fn py_get_product_id(&self) -> String {
        self.product_id.clone()
    }

    #[getter(size)]
    pub fn py_get_size(&self) -> String {
        self.size.clone()
    }

    #[getter(quality)]
    pub fn py_get_quality(&self) -> f32 {
        self.quality
    }

    #[pyo3(name = "__repr__")]
    pub fn py_repr(&self) -> String {
        self.to_string()
    }
}
