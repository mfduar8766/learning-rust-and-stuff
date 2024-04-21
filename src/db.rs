use anyhow::Error;
use mongodb::{options::ClientOptions, Client, Database};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use tracing::{error, info};
use crate::CONFIG;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    password: String,
}

impl User {
    fn new() -> Self {
        return Self {
            email: String::from("test@tester12.com"),
            password: String::from("123"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            dob: String::from("1992/06/07"),
        };
    }
}

#[derive(Debug)]
pub struct Iteniary {}

impl Iteniary {
    fn new() -> Self {
        return Self {};
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Db {
    pub user: User,
    pub iteniary: Vec<Iteniary>,
    pub db: Option<Database>,
}

impl Db {
    pub fn new(db: Option<Database>) -> Self {
        return Self {
            user: User::new(),
            iteniary: vec![Iteniary::new()],
            db,
        };
    }
    pub fn authenticate(&self, email: &str, password: &str) -> bool {
        if email.len() == 0 || password.len() == 0 {
            return false;
        } else if email.contains("test@tester12.com") && password.contains("123") {
            return true;
        } else {
            return false;
        }
    }
}

pub async fn create_db(file: &str) -> Result<Option<Database>, Error> {
    if !CONFIG.lock().unwrap().get_envs().use_db {
        return Ok(None);
    }
    let mut client_options =
        ClientOptions::parse(CONFIG.lock().unwrap().get_envs().mongo_url.as_str()).await?;
    client_options.app_name = Some(CONFIG.lock().unwrap().service_name.as_str().to_owned());
    let client = match Client::with_options(client_options) {
        Ok(c) => c,
        Err(e) => {
            error!("error creating DB connection: {}", e);
            return Err(e.into());
        }
    };
    let db = client.database(&CONFIG.lock().unwrap().get_envs().db_name);
    let _ = db
        .create_collection(&CONFIG.lock().unwrap().get_envs().db_collection, None)
        .await?;
    if fs::metadata(file).is_ok() {
        let user: User = serde_json::from_str(file).unwrap();
        info!("USER: {:?}", user);
    }
    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    return Ok(Some(db));
}
