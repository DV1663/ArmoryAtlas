use chrono::{Datelike, NaiveDate};
use rand::Rng;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "python-db", derive(pyo3::FromPyObject))]
#[cfg_attr(feature = "mysql-db", derive(sqlx::FromRow))]
pub struct SSN {
    value: String,
}

impl From<SSN> for String {
    fn from(value: SSN) -> Self {
        value.value
    }
}

impl SSN {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn generate_random_date() -> String {
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

        // Format the date as "YYMMDD"
        format!(
            "{:02}{:02}{:02}",
            date.year() % 100,
            date.month(),
            date.day()
        )
    }

    pub fn new_random(gender: bool) -> Self {
        let date_part = Self::generate_random_date();

        // Generate a random number between 0 and 9
        let random_part_1 = rand::thread_rng().gen_range(0..=9).to_string();
        let random_part_2 = rand::thread_rng().gen_range(0..=9).to_string();
        let random_part = format!("{}{}", random_part_1, random_part_2);

        let gender_part = if gender {
            // generate a random uneaven number between 0 and 9
            let uneaven = [1, 3, 5, 7, 9];

            uneaven[rand::thread_rng().gen_range(0..=4)].to_string()
        } else {
            let even = [0, 2, 4, 6, 8];

            even[rand::thread_rng().gen_range(0..=4)].to_string()
        };

        let value = format!("{}{}{}", &date_part, &random_part, &gender_part);

        let ssn = SSN::generate_control_digit(&value);

        Self::new(format!(
            "{}-{}{}{}",
            date_part, random_part, gender_part, ssn
        ))
    }

    pub fn generate_control_digit(first_nine: &str) -> String {
        // cast the string to a vector of signed integer digits

        let digits: Vec<i32> = first_nine
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let mut new_digits = Vec::new();
        for (i, digit) in digits.iter().enumerate() {
            if digit == &0 {
                continue;
            }
            if (i + 1) % 2 == 0 {
                let new_digit = *digit;
                if new_digit != 0 {
                    new_digits.push(new_digit);
                }
            } else {
                let doubled_digit = digit * 2;
                if doubled_digit > 9 {
                    // split into two sepperate digits
                    let tmp = doubled_digit.to_string();
                    let (first, second) = tmp.split_at(1);

                    // cast to u32 and check if is not 0 if its not 0 add to array

                    let first = first.parse::<i32>().unwrap();
                    let second = second.parse::<i32>().unwrap();

                    if first != 0 {
                        new_digits.push(first);
                    }

                    if second != 0 {
                        new_digits.push(second);
                    }
                } else if doubled_digit != 0 {
                    new_digits.push(doubled_digit);
                }
            }
        }

        let total = (10 - new_digits.iter().sum::<i32>() % 10) % 10;

        total.to_string()
    }
}