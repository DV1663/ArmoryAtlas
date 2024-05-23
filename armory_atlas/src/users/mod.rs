pub mod ssn;
#[cfg(feature = "python-db")]
mod python_impl;

#[cfg(feature = "python-db")]
use crate::python_db_handler::DBHandlerPy as DBHandler;
use anyhow::Result;
use rand::Rng;
use crate::cli::InsertUserArgs;
use crate::users::ssn::SSN;

pub fn insert_users(db_handler: &DBHandler, num_users: usize) -> Result<()> {
    let users = generate_users(num_users);

    for user in users {
        db_handler.insert_user(user)?;
    }

    Ok(())
}

fn generate_users(num_users: usize) -> Vec<User> {
    let mut users = Vec::new();

    for _ in 0..num_users {
        users.push(User::new_random());
    }

    users
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mysql-db", derive(sqlx::FromRow))]
#[cfg_attr(feature = "python-db", pyo3::pyclass)]
pub struct User {
    pub ssn: String,
    pub name: String,
}

impl User {
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
        let last_names = [
            "Smith",
            "Johnson",
            "Williams",
            "Brown",
            "Jones",
            "Garcia",
            "Miller",
            "Davis",
            "Rodriguez",
            "Martinez",
        ];

        // Get a random number generator
        let mut rng = rand::thread_rng();

        let first_name = {
            let male_names: Vec<&str> = vec![
                "Liam",
                "Noah",
                "Oliver",
                "Elijah",
                "William",
                "James",
                "Benjamin",
                "Lucas",
                "Henry",
                "Alexander",
            ];

            let female_names: Vec<&str> = vec![
                "Olivia",
                "Emma",
                "Ava",
                "Charlotte",
                "Sophia",
                "Amelia",
                "Isabella",
                "Mia",
                "Evelyn",
                "Harper",
            ];

            if gender {
                male_names[rng.gen_range(0..male_names.len())].to_string()
            } else {
                female_names[rng.gen_range(0..female_names.len())].to_string()
            }
        };

        let last_name = last_names[rng.gen_range(0..last_names.len())].to_string();

        (first_name, last_name)
    }
}

impl From<InsertUserArgs> for User {
    fn from(insert_user_args: InsertUserArgs) -> Self {
        Self {
            ssn: insert_user_args.ssn,
            name: insert_user_args.name,
        }
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
        let user = User::new_random();
        println!("{:?}", user);
    }
}
