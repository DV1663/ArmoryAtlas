use chrono::NaiveDate;
use pyo3::FromPyObject;
use rand::Rng;
use sqlx::FromRow;
use sqlx_mysql::MySqlPool;

use crate::db_handler::DBHandler;

pub async fn insert_leandings(pool: &MySqlPool, num_leandings: usize) -> anyhow::Result<()> {
    let leandings = generate_leandings(num_leandings)?;

    for leanding in leandings {
        sqlx::query("INSERT INTO Leandings (LeandingID, UserID, ProductID, BorrowingDate, ReturnDate) VALUES (UUID_TO_BIN(UUID()), ?, ?, ?, ?)")
            .bind(&leanding.user_id)
            .bind(&leanding.product_id)
            .bind(&leanding.borrowing_date)
            .bind(&leanding.return_date)
            .execute(pool)
            .await?;
    }
    Ok(())
}

fn generate_leandings(num_leandings: usize) -> anyhow::Result<Vec<Loans>> {
    let mut leandings = Vec::new();

    for _ in 0..num_leandings {
        leandings.push(Loans::new_random()?);
    }

    Ok(leandings)
}

#[derive(Debug, FromRow, FromPyObject)]
pub struct Loans {
    pub leanding_id: String,
    pub user_id: String,
    pub product_id: String,
    pub borrowing_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
}

impl Loans {
    pub fn new(
        user_id: String,
        product_id: String,
        borrowing_date: NaiveDate,
        return_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            leanding_id: String::new(),
            user_id,
            product_id,
            borrowing_date,
            return_date,
        }
    }

    pub fn generate_random_date() -> NaiveDate {
        let mut rng = rand::thread_rng();

        // Define the range of years, months, and days
        // For example, for years between 1950 and 1999
        let year = rng.gen_range(1950..2006);
        let month = rng.gen_range(1..=12);
        let day = match month {
            4 | 6 | 9 | 11 => rng.gen_range(1..=30),
            2 => {
                // Check if it's a leap year
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    rng.gen_range(1..=29)
                } else {
                    rng.gen_range(1..=28)
                }
            }
            _ => rng.gen_range(1..=31),
        };

        // Create a NaiveDate object from the generated year, month, and day
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        date
    }

    pub fn new_random() -> anyhow::Result<Self> {
        let db = DBHandler::new()?;
        let product = db.get_rand_item()?;
        let borrowing_date = Loans::generate_random_date();
        let user = db.get_rand_user()?;

        Ok(Self {
            leanding_id: String::new(),
            user_id: user.ssn,
            product_id: product.item_id,
            borrowing_date,
            return_date: None,
        })
    }
}
