use std::error::Error;
use std::fs;

use super::{Config, search};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;
    
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}