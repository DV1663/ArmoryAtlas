use chrono::NaiveDate;
use pyo3::pymethods;

use crate::leandings::Loans;

#[pymethods]
impl Loans {
    #[new]
    pub fn py_new(
        ssn: String,
        product_id: String,
        borrowing_date: NaiveDate,
        return_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            leanding_id: String::new(),
            ssn,
            item_id: product_id,
            borrowing_date,
            return_date,
        }
    }

    #[staticmethod]
    pub fn py_generate_random_date(start_date: Option<NaiveDate>) -> NaiveDate {
        Self::generate_random_date(start_date)
    }

    #[staticmethod]
    pub fn py_new_random() -> anyhow::Result<Self> {
        Self::new_random()
    }

    #[pyo3(name = "__repr__")]
    pub fn py_repr(&self) -> String {
        format!(
            "Lending ID: {}\nUser ID: {}\nProduct ID: {}\nBorrowing Date: {}\nReturn Date: {:?}",
            self.leanding_id, self.ssn, self.item_id, self.borrowing_date, self.return_date
        )
    }

    #[getter(id)]
    pub fn py_get_leanding_id(&self) -> String {
        self.leanding_id.clone()
    }

    #[getter(ssn)]
    pub fn py_get_ssn(&self) -> String {
        self.ssn.clone()
    }

    #[getter(item_id)]
    pub fn py_get_product_id(&self) -> String {
        self.item_id.clone()
    }

    #[getter(borrowing_date)]
    pub fn py_get_borrowing_date(&self) -> NaiveDate {
        self.borrowing_date
    }

    #[getter(return_date)]
    pub fn py_get_return_date(&self) -> Option<NaiveDate> {
        self.return_date
    }
}