use crate::CONFIG;
use anyhow::Error;
use mongodb::{options::ClientOptions, Client, Database};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use tracing::{error, info};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    password: String,
}

impl Default for User {
    fn default() -> Self {
        return Self {
            email: String::from("test@tester12.com"),
            password: String::from("123"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            dob: String::from("06/07/1992"),
        };
    }
}

impl User {
    fn new() -> Self {
        return Self {
            email: String::from("test@tester12.com"),
            password: String::from("123"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            dob: String::from("06/07/1992"),
        };
    }
}

#[derive(Debug)]
pub struct Activity {
    pub time: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Days {
    pub activity: Activity,
}

#[derive(Debug)]
pub struct Itinerary {
    pub destination: String,
    pub image: String,
    pub departure: String,
    pub arrival: String,
    pub over_all_budget: String,
}

impl Itinerary {
    pub fn new() -> Self {
        return Self {
            destination: String::from("Germany"),
            image: String::from("Germany.png"),
            departure: String::from("May 23, 20204 8:00AM"),
            arrival: String::from("May 23, 2024 4:00PM"),
            over_all_budget: String::from("$100,000.00"),
        };
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Db {
    pub user: User,
    pub iteniary: Vec<Itinerary>,
    pub db: Option<Database>,
    is_authenticated: bool,
}

impl Db {
    pub fn new(db: Option<Database>) -> Self {
        return Self {
            user: User::new(),
            iteniary: vec![Itinerary::new()],
            db,
            is_authenticated: false,
        };
    }
    pub fn authenticate(&mut self, email: &str, password: &str) -> bool {
        if email.len() == 0 || password.len() == 0 {
            return false;
        } else if email.contains("test@tester12.com") && password.contains("123") {
            self.is_authenticated = true;
            return true;
        } else {
            return false;
        }
    }
    pub fn is_authenticated(&self) -> bool {
        return self.is_authenticated;
    }
    pub fn set_is_authenticated(&mut self) {
        self.is_authenticated = false;
    }
    pub fn get_user(&mut self) -> &mut User {
        return &mut self.user;
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
