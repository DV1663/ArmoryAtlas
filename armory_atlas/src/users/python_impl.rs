use crate::users::User;

#[pyo3::pymethods]
impl User {
    #[new]
    pub fn py_new(ssn: String, name: String) -> Self {
        Self {
            ssn,
            name,
        }
    }
    
    #[staticmethod]
    #[pyo3(name = "new_random")]
    pub fn py_new_random() -> Self {
        Self::new_random()
    }

    #[getter(ssn)]
    pub fn py_get_ssn(&self) -> String {
        self.ssn.clone()
    }

    #[getter(name)]
    pub fn py_get_name(&self) -> String {
        self.name.clone()
    }

    #[pyo3(name = "__repr__")]
    pub fn py_repr(&self) -> String {
        format!("{:?}", self)
    }

    #[pyo3(name = "__str__")]
    pub fn py_str(&self) -> String {
        format!("SSN: {}\nName: {}", self.ssn, self.name)
    }
}