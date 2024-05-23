use prettytable::{row, Row, Table};
use pyo3::{pyclass, pymethods, FromPyObject};

#[derive(FromPyObject)]
pub struct PyNumberBorrow {
    pub ssn: String,
    pub name: String,
    pub tot_borrowes: i64,
    pub curr_borrowes: i64,
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct NumberBorrow {
    pub ssn: String,
    pub name: String,
    pub tot_borrowes: i64,
    pub curr_borrowes: i64,
}

#[pymethods]
impl NumberBorrow {
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
pub struct NumberBorrows(Vec<NumberBorrow>);

#[pymethods]
impl NumberBorrows {
    #[getter(borrows)]
    fn get_number_borrows(&self) -> Vec<NumberBorrow> {
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

impl From<NumberBorrow> for PyNumberBorrow {
    fn from(number_borrow: NumberBorrow) -> Self {
        Self {
            ssn: number_borrow.ssn,
            name: number_borrow.name,
            tot_borrowes: number_borrow.tot_borrowes,
            curr_borrowes: number_borrow.curr_borrowes,
        }
    }
}

impl From<PyNumberBorrow> for NumberBorrow {
    fn from(py_number_borrow: PyNumberBorrow) -> Self {
        Self {
            ssn: py_number_borrow.ssn,
            name: py_number_borrow.name,
            tot_borrowes: py_number_borrow.tot_borrowes,
            curr_borrowes: py_number_borrow.curr_borrowes,
        }
    }
}

impl From<Vec<NumberBorrow>> for NumberBorrows {
    fn from(number_borrows: Vec<NumberBorrow>) -> Self {
        Self(number_borrows)
    }
}

impl From<NumberBorrows> for Vec<NumberBorrow> {
    fn from(number_borrows: NumberBorrows) -> Self {
        number_borrows.0
    }
}

impl From<NumberBorrow> for NumberBorrows {
    fn from(number_borrow: NumberBorrow) -> Self {
        Self(vec![number_borrow])
    }
}

impl std::ops::Index<usize> for NumberBorrows {
    type Output = NumberBorrow;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<NumberBorrow> for Row {
    fn from(value: NumberBorrow) -> Self {
        row![
            value.ssn,
            value.name,
            value.tot_borrowes,
            value.curr_borrowes
        ]
    }
}

impl From<&NumberBorrow> for Row {
    fn from(value: &NumberBorrow) -> Self {
        row![
            value.ssn,
            value.name,
            value.tot_borrowes,
            value.curr_borrowes
        ]
    }
}

impl From<NumberBorrows> for Table {
    fn from(number_borrows: NumberBorrows) -> Self {
        let mut table = Table::new();
        table.add_row(row!["SSN", "Name", "Total Borrows", "Current Borrows"]);
        for borrow in number_borrows.0 {
            table.add_row((&borrow).into());
        }
        table
    }
}
