/*
    class InStockSize:
        def __init__(self, product_id, size, tot_in):
            self.product_id = product_id
            self.size = size
            self.tot_in = tot_in
*/

use std::ops::Index;
use prettytable::{Row, row, Table};
use pyo3::{FromPyObject, pyclass};

#[derive(FromPyObject)]
#[pyclass]
pub struct InStockSize {
    pub product_id: String,
    pub product_name: String,
    pub size: String,
    pub tot_in: i32,
}

#[pyclass]
pub struct InStockSizes(Vec<InStockSize>);

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
        row![value.product_id, value.product_name, value.size, value.tot_in]
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