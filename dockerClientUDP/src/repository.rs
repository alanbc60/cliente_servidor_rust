use crate::config;
use postgres::{Client,NoTls};

mod product;

pub struct Repository{
    db: Client,
}

impl Repository{
    pub fn new() -> Repository {
        Repository { 
            db: Client::connect(config::DB_CONFIG, NoTls).unwrap(), 
        }
    }
}