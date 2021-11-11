use reqwest::{blocking::Client, redirect};
use std::{env, time::Duration};

mod error;
pub use error::Error;

mod model;

mod subdomains;
use model::Subdomain;


fn main() -> Result<(), anyhow::Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(error::CliUsage.into());
    }

    
}