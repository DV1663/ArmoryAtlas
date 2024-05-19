use std::ops::{Index};
use prettytable::{Row, row, Table};
use pyo3::{FromPyObject, pyclass, pymethods};

#[derive(FromPyObject)]
pub struct PyDetailedLoan {
    pub lending_id: String,
    pub ssn: String,
    pub name: String,
    pub item_id: String,
    pub product_name: String,
    pub size: String,
    pub borrow_date: String,
    pub return_date: Option<String>,
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct DetailedLoan {
    pub lending_id: String,
    pub ssn: String,
    pub name: String,
    pub item_id: String,
    pub product_name: String,
    pub size: String,
    pub borrow_date: String,
    pub return_date: Option<String>,
}

#[pymethods]
impl DetailedLoan {
    #[pyo3(name = "__repr__")]
    pub fn repr(&self) -> String {
        format!("{:?}", self)
    }
    
    #[pyo3(name = "__str__")]
    pub fn str(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
#[pyclass]
pub struct DetailedLoans(Vec<DetailedLoan>);

#[pymethods]
impl DetailedLoans {
    #[getter(loans)]
    fn get_detailed_loans(&self) -> Vec<DetailedLoan> {
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

impl From<DetailedLoan> for PyDetailedLoan {
    fn from(detailed_loan: DetailedLoan) -> Self {
        Self {
            lending_id: detailed_loan.lending_id,
            ssn: detailed_loan.ssn,
            name: detailed_loan.name,
            item_id: detailed_loan.item_id,
            product_name: detailed_loan.product_name,
            size: detailed_loan.size,
            borrow_date: detailed_loan.borrow_date,
            return_date: detailed_loan.return_date,
        }
    }
}

impl From<PyDetailedLoan> for DetailedLoan {
    fn from(py_detailed_loan: PyDetailedLoan) -> Self {
        Self {
            lending_id: py_detailed_loan.lending_id,
            ssn: py_detailed_loan.ssn,
            name: py_detailed_loan.name,
            item_id: py_detailed_loan.item_id,
            product_name: py_detailed_loan.product_name,
            size: py_detailed_loan.size,
            borrow_date: py_detailed_loan.borrow_date,
            return_date: py_detailed_loan.return_date,
        }
    }
}



impl From<Vec<DetailedLoan>> for DetailedLoans {
    fn from(detailed_loans: Vec<DetailedLoan>) -> Self {
        Self(detailed_loans)
    }
}

impl From<DetailedLoans> for Vec<DetailedLoan> {
    fn from(detailed_loans: DetailedLoans) -> Self {
        detailed_loans.0
    }
}

impl From<DetailedLoan> for DetailedLoans {
    fn from(detailed_loan: DetailedLoan) -> Self {
        Self(vec![detailed_loan])
    }
}

impl Index<usize> for DetailedLoans {
    type Output = DetailedLoan;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<DetailedLoan> for Row {
    fn from(value: DetailedLoan) -> Self {
        row![
            value.lending_id,
            value.ssn,
            value.name,
            value.item_id,
            value.product_name,
            value.size,
            value.borrow_date,
            value.return_date.unwrap_or("Not Returned yet".to_string())
        ]
    }
}

impl From<&DetailedLoan> for Row {
    fn from(value: &DetailedLoan) -> Self {
        row![
            value.lending_id,
            value.ssn,
            value.name,
            value.item_id,
            value.product_name,
            value.size,
            value.borrow_date,
            value.return_date.clone().unwrap_or("Not Returned yet".to_string())
        ]
    }
}

impl From<DetailedLoans> for Table {
    fn from(detailed_loans: DetailedLoans) -> Self {
        let mut table = Table::new();
        table.add_row(row!["Lending ID", "SSN", "Name", "Item ID", "Product Name", "Size", "Borrowing Date", "Return Date"]);
        for loan in detailed_loans.0 {
            table.add_row((&loan).into());
        }
        table
    }
}


