use std::io::Read;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub mod config;
pub mod cli;
pub mod password_handler;
pub mod products;
pub mod items;

pub const CONFIG_FILE: &str = ".config/armoryatlas/config.toml";


