use chrono::{NaiveDate, Datelike};
use pyo3::{FromPyObject, pymethods};
use rand::Rng;

use crate::db_handler::DBHandler;

pub fn insert_leandings(db_handler: &DBHandler, num_leandings: usize) -> anyhow::Result<()> {
    for _ in 0..num_leandings {
        let leanding = Loans::new_random()?;
        db_handler.insert_loan(leanding)?;
    }
    
    Ok(())
}

#[derive(Debug, FromPyObject)]
#[cfg_attr(feature = "rs-db", derive(sqlx::FromRow))]
#[pyo3::pyclass]
pub struct Loans {
    pub leanding_id: String,
    pub user_id: String,
    pub item_id: String,
    pub borrowing_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
}

#[pymethods]
impl Loans {
    #[new]
    pub fn new(
        user_id: String,
        product_id: String,
        borrowing_date: NaiveDate,
        return_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            leanding_id: String::new(),
            user_id,
            item_id: product_id,
            borrowing_date,
            return_date,
        }
    }

    #[staticmethod]
    pub fn generate_random_date(start_date: Option<NaiveDate>) -> NaiveDate {
        let mut rng = rand::thread_rng();
        let start_year = start_date.map(|d| d.year()).unwrap_or(1950);

        // Define the range of years, for example, from start_year to 2024
        let year = rng.gen_range(start_year..2024);
        let month = rng.gen_range(1..=12);
        let day = match month {
            4 | 6 | 9 | 11 => rng.gen_range(1..=30),
            2 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                rng.gen_range(1..=29)
            } else {
                rng.gen_range(1..=28)
            },
            _ => rng.gen_range(1..=31),
        };

        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[staticmethod]
    pub fn new_random() -> anyhow::Result<Self> {
        let db = DBHandler::new()?;
        let product = db.get_rand_item()?;
        let borrowing_date = Loans::generate_random_date(None);
        let user = db.get_rand_user()?;
        
        // randomly choose if the item is reutrned or not
        let mut rng = rand::thread_rng();
        let return_date = if rng.gen_bool(0.2) {
            Some(Loans::generate_random_date(Some(borrowing_date)))
        } else {
            None
        };

        Ok(Self {
            leanding_id: String::new(),
            user_id: user.ssn,
            item_id: product.item_id,
            borrowing_date,
            return_date,
        })
    }
    
    #[pyo3(name = "__repr__")]
    pub fn repr(&self) -> String {
        format!(
            "Lending ID: {}\nUser ID: {}\nProduct ID: {}\nBorrowing Date: {}\nReturn Date: {:?}",
            self.leanding_id, self.user_id, self.item_id, self.borrowing_date, self.return_date
        )
    }
    
    #[getter(id)]
    pub fn get_leanding_id(&self) -> String {
        self.leanding_id.clone()
    }
    
    #[getter(user_id)]
    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }
    
    #[getter(item_id)]
    pub fn get_product_id(&self) -> String {
        self.item_id.clone()
    }
    
    #[getter(borrowing_date)]
    pub fn get_borrowing_date(&self) -> NaiveDate {
        self.borrowing_date
    }
    
    #[getter(return_date)]
    pub fn get_return_date(&self) -> Option<NaiveDate> {
        self.return_date
    }
}
