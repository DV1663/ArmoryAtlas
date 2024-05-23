use prettytable::{row, Row, Table};
use pyo3::{pyclass, pymethods, FromPyObject};
use std::ops::Index;

#[derive(FromPyObject, Debug)]
#[pyclass]
pub struct InStockSize {
    pub product_id: String,
    pub product_name: String,
    pub size: String,
    pub tot_in: i32,
}

#[pymethods]
impl InStockSize {
    #[pyo3(name = "__repr__")]
    pub fn repr(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
#[pyclass]
pub struct InStockSizes(Vec<InStockSize>);

#[pymethods]
impl InStockSizes {
    #[getter(sizes)]
    fn get_in_stock_sizes(&self) -> Vec<InStockSize> {
        let mut tmp = Vec::new();
        for in_stock_size in &self.0 {
            tmp.push(InStockSize {
                product_id: in_stock_size.product_id.clone(),
                product_name: in_stock_size.product_name.clone(),
                size: in_stock_size.size.clone(),
                tot_in: in_stock_size.tot_in,
            })
        }

        tmp
    }
    #[pyo3(name = "__repr__")]
    pub fn repr(&self) -> String {
        format!("{:?}", self)
    }
}

impl Index<usize> for InStockSizes {
    type Output = InStockSize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<Vec<InStockSize>> for InStockSizes {
    fn from(in_stock_sizes: Vec<InStockSize>) -> Self {
        Self(in_stock_sizes)
    }
}

impl From<InStockSizes> for Vec<InStockSize> {
    fn from(in_stock_sizes: InStockSizes) -> Self {
        in_stock_sizes.0
    }
}

impl From<InStockSize> for Row {
    fn from(value: InStockSize) -> Self {
        row![value.product_id, value.size, value.tot_in]
    }
}

impl From<&InStockSize> for Row {
    fn from(value: &InStockSize) -> Self {
        row![
            value.product_id,
            value.product_name,
            value.size,
            value.tot_in
        ]
    }
}

impl From<InStockSize> for InStockSizes {
    fn from(in_stock_size: InStockSize) -> Self {
        Self(vec![in_stock_size])
    }
}

impl From<InStockSizes> for Table {
    fn from(in_stock_sizes: InStockSizes) -> Self {
        let mut table = Table::new();
        table.add_row(row!["Product ID", "Product Name", "Size", "In Stock"]);
        for in_stock_size in in_stock_sizes.0 {
            table.add_row((&in_stock_size).into());
        }
        table
    }
}
