use anyhow::{self, Ok};
mod error;
mod model;
mod subdomain;
use reqwest::Client;
use thiserror::Error;
#[derive(Error, Debug, Clone)]

pub enum Error {
    #[error("Usage: tricoder <Domain>")]
    CliUsage,
}

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    Ok(())
}
