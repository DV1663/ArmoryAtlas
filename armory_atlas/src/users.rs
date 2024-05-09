use chrono::{NaiveDate, Datelike};
use pyo3::FromPyObject;
use rand::Rng;
use sqlx::FromRow;
use sqlx_mysql::MySqlPool;
use anyhow::Result;

pub async fn insert_users(pool: &MySqlPool, num_users: usize) -> Result<()> {
    let users = generate_users(num_users);
    
    for user in users {
        sqlx::query("INSERT INTO Users (SSN, Name) VALUES (?, ?)")
            .bind(&user.ssn)
            .bind(&user.name)
            .execute(pool)
            .await?;
    }
    Ok(())
}

fn generate_users(num_users: usize) -> Vec<Users> {
    let mut users = Vec::new();
    
    for _ in 0..num_users {
        users.push(Users::new_random());
    }

    users
}

#[derive(Debug, FromRow, FromPyObject)]
pub struct Users {
    pub ssn: String,
    pub name: String,
}

impl Users {
    pub fn new(name: String, ssn: SSN) -> Self {
        
        Self {
            ssn: ssn.into(),
            name
        }
    }

    pub fn new_random() -> Self {
        let gender = Self::generate_random_gender();
        let (first_name, last_name) = Self::generate_random_name(gender);
        Self {
            ssn: SSN::new_random(gender).into(),
            name: format!("{} {}", first_name, last_name),
        }
    }

    fn generate_random_gender() -> bool {
        let mut rng = rand::thread_rng();
        // Randomly generates true or false, where true represents a man and false represents a woman.
        rng.gen_bool(0.5) // 50% chance for each gender
    }

    fn generate_random_name(gender: bool) -> (String, String) {
        let first_names = vec![
            "James", "Mary", "John", "Patricia", "Robert", "Jennifer",
            "Michael", "Linda", "William", "Elizabeth"
        ];
        let last_names = vec![
            "Smith", "Johnson", "Williams", "Brown", "Jones",
            "Garcia", "Miller", "Davis", "Rodriguez", "Martinez"
        ];

        // Get a random number generator
        let mut rng = rand::thread_rng();

        // Choose a random first name and last name from the lists
        let first_name = first_names[rng.gen_range(0..first_names.len())].to_string();
        let last_name = last_names[rng.gen_range(0..last_names.len())].to_string();

        (first_name, last_name)
    }
}

#[derive(Debug, Clone, FromRow, FromPyObject)]
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
        format!("{:02}{:02}{:02}", date.year() % 100, date.month(), date.day())
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
            let random_uneaven = uneaven[rand::thread_rng().gen_range(0..=4)].to_string();
            random_uneaven
        } else {
            let even = [0, 2, 4, 6, 8];
            let random_even = even[rand::thread_rng().gen_range(0..=4)].to_string();
            random_even
        };
        
        let value = format!("{}{}{}", &date_part, &random_part, &gender_part);
        
        let ssn = SSN::generate_control_digit(&value);
        
        Self::new(format!("{}-{}{}{}", date_part, random_part, gender_part, ssn))
    }
    
    pub fn generate_control_digit(first_nine: &String) -> String {
        // cast the string to a vector of signed integer digits
        
        let digits: Vec<i32> = first_nine.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        
        let mut new_digits = Vec::new();
        for (i, digit) in digits.iter().enumerate() {
            if digit == &0 {
                continue;
            }
            if (i + 1) % 2 == 0 {
                let new_digit = digit * 1;
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

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test_ssn() {
        let value = "001227828".to_string();
        let ssn = SSN::generate_control_digit(&value);
        println!("{}", ssn);
    }
    
    #[test]
    fn test_new_random() {
        let user = Users::new_random();
        println!("{:?}", user);
    }
}