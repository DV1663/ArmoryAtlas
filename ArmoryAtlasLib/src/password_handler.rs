use anyhow::Result;
use keyring::Entry;
use std::io;

fn get_keyring(usr: &str, host: &str) -> Result<Entry> {
    Ok(Entry::new("rustic_sql_manager", format!("{usr}@{host}").as_str())?)
}

pub fn get_db_pass(usr: &str, host: &str) -> Result<String> {
    match get_keyring(usr, host) {
        Ok(entry) => {
            match entry.get_password() {
                Ok(pass) => Ok(pass),
                Err(_) => {
                    let pass = rpassword::prompt_password("Enter User password: ")?;
                    // does the user want to save the password? y/n
                    loop {
                        println!("Do you want to save your password? (yes|y/no|n)");
                        let mut save_password = String::new();
                        io::stdin()
                            .read_line(&mut save_password)
                            .expect("Failed to read line");
                        let save_password = save_password.trim().to_lowercase();

                        match save_password.as_str() {
                            "yes" | "y" => {
                                println!("Saving password...");
                                if entry.set_password(&pass).is_err() {
                                    eprintln!("No Native Keyring found. Please install one and try again.");
                                }
                                break;
                            }
                            "no" | "n" => {
                                println!("Not saving password.");
                                break;
                            }
                            _ => println!("Invalid input. Please enter 'yes' or 'no'."),
                        }
                    }
                    Ok(pass)
                }
            }
        }
        Err(_) => {
            eprintln!("No Native Keyring found. Please install one and try again.");
            let pass = rpassword::prompt_password("Enter User password: ")?;
            Ok(pass)
        }
    }
}