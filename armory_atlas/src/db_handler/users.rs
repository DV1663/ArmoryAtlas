use std::ops::Index;
use prettytable::{Row, row, Table};
use pyo3::{FromPyObject, pyclass, pymethods};
use rayon::prelude::*;
use crate::users::User;

#[derive(FromPyObject)]
pub struct PyUser {
    pub ssn: String,
    pub name: String,
}

#[derive(Debug)]
#[pyclass]
pub struct Users(Vec<User>);

#[pymethods]
impl Users {
    #[getter(users)]
    fn get_users(&self) -> Vec<User> {
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

impl From<PyUser> for User {
    fn from(py_user: PyUser) -> Self {
        Self {
            ssn: py_user.ssn,
            name: py_user.name,
        }
    }
}

impl From<User> for PyUser {
    fn from(user: User) -> Self {
        Self {
            ssn: user.ssn,
            name: user.name,
        }
    }
}

impl From<Vec<PyUser>> for Users {
    fn from(py_users: Vec<PyUser>) -> Self {
        Self(py_users
            .into_par_iter()
            .map(User::from)
            .collect())
    }
}

impl From<Users> for Vec<PyUser> {
    fn from(users: Users) -> Self {
        users.0
            .into_par_iter()
            .map(|user| user.into())
            .collect()
    }
}

impl From<Vec<User>> for Users {
    fn from(users: Vec<User>) -> Self {
        Self(users)
    }
}

impl From<Users> for Vec<User> {
    fn from(users: Users) -> Self {
        users.0
    }
}

impl From<User> for Users {
    fn from(user: User) -> Self {
        Self(vec![user])
    }
}

impl From<User> for Row {
    fn from(user: User) -> Self {
        row![user.ssn, user.name]
    }
}

impl Index<usize> for Users {
    type Output = User;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<Users> for Table {
    fn from(users: Users) -> Self {
        let mut table = Table::new();
        table.add_row(row!["SSN", "Name"]);
        for user in users.0 {
            table.add_row(user.into());
        }
        
        table
    }
}

