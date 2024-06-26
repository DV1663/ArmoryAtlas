#[cfg(feature = "python-db")]
mod python_impl;

use crate::cli::InsertLoanArgs;
use chrono::{Datelike, NaiveDate};
use rand::Rng;

#[cfg(feature = "python-db")]
use crate::python_db_handler::DBHandlerPy as DBHandler;

pub fn insert_leandings(db_handler: &DBHandler, num_leandings: usize) -> anyhow::Result<()> {
    for _ in 0..num_leandings {
        let leanding = Loans::new_random()?;
        db_handler.insert_loan(leanding)?;
    }

    Ok(())
}

#[derive(Debug)]
#[cfg_attr(feature = "python-db", derive(pyo3::FromPyObject))]
#[cfg_attr(feature = "mysql-db", derive(sqlx::FromRow))]
#[cfg_attr(feature = "python-db", pyo3::pyclass)]
pub struct Loans {
    pub leanding_id: String,
    pub ssn: String,
    pub item_id: String,
    pub borrowing_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
}

impl Loans {
    pub fn new(
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

    pub fn generate_random_date(start_date: Option<NaiveDate>) -> NaiveDate {
        let mut rng = rand::thread_rng();
        let start_year = start_date.map(|d| d.year()).unwrap_or(1950);

        // Define the range of years, for example, from start_year to 2024
        let year = rng.gen_range(start_year..2024);
        let month = rng.gen_range(1..=12);
        let day = match month {
            4 | 6 | 9 | 11 => rng.gen_range(1..=30),
            2 => {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    rng.gen_range(1..=29)
                } else {
                    rng.gen_range(1..=28)
                }
            }
            _ => rng.gen_range(1..=31),
        };

        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

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
            ssn: user.ssn,
            item_id: product.item_id,
            borrowing_date,
            return_date,
        })
    }
}

impl From<InsertLoanArgs> for Loans {
    fn from(insert_loan_args: InsertLoanArgs) -> Self {
        Self {
            leanding_id: String::new(),
            ssn: insert_loan_args.ssn,
            item_id: insert_loan_args.item_id,
            borrowing_date: insert_loan_args.borrow_date,
            return_date: insert_loan_args.return_date,
        }
    }
}
