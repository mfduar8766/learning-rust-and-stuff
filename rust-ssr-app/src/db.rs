use crate::CONFIG;
use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions};
use sqlx::{Pool, Postgres};
use std::thread::sleep;
use std::time::Duration;
use tracing::{error, info, warn};

#[derive(Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Users {
    id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub user_name: String,
    password: String,
}

#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Itinieary {
    pub id: i32,
    pub destination: String,
    pub resource_id: String,
    pub departure: chrono::DateTime<chrono::Utc>,
    pub arrival: chrono::DateTime<chrono::Utc>,
    pub over_all_budget: String,
    pub user_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Db {
    pub user: Users,
    pub iteniary: Vec<Itinieary>,
    is_authenticated: bool,
    db_instance: Option<Pool<Postgres>>,
}

impl Db {
    pub fn new() -> Self {
        return Self {
            is_authenticated: false,
            user: Users::default(),
            iteniary: vec![Itinieary::default()],
            db_instance: None,
        };
    }
    pub fn set_db(&mut self, db: Pool<Postgres>) {
        self.db_instance = Some(db);
    }
    pub async fn authenticate(&mut self, email: &str, password: &str) -> Result<(), Error> {
        if email.len() == 0 || password.len() == 0 {
            return Ok(());
        }
        let db = &self.db_instance;
        let c = CONFIG.get().unwrap();
        match db {
            Some(db) => {
                let users = &c.get_envs().db_users;
                let check_user_name_and_password =
                    format!("SELECT * FROM {} WHERE email = $1 AND password = $2", users);
                match sqlx::query_as::<_, Users>(&check_user_name_and_password)
                    .bind(email)
                    .bind(password)
                    .fetch_one(db)
                    .await
                {
                    Ok(users) => {
                        let itineary = &c.get_envs().db_itineary;
                        self.user = users;
                        let get_itinearies =
                            format!("SELECT * FROM {} WHERE user_id = $1", itineary);
                        self.is_authenticated = true;
                        match sqlx::query_as::<_, Itinieary>(&get_itinearies)
                            .bind(self.user.id)
                            .fetch_all(db)
                            .await
                        {
                            Ok(itineary) => {
                                self.iteniary = itineary;
                                return Ok(());
                            }
                            Err(e) => {
                                warn!("db::authenticate()::error getting itinearies:{}", e);
                                return Err(e.into());
                            }
                        }
                    }
                    Err(e) => {
                        warn!("db::authenticate()::users::error:{}", e);
                        return Err(e.into());
                    }
                }
            }
            None => todo!(),
        };
    }
    pub fn set_is_authenticated(&mut self, value: bool) {
        self.is_authenticated = value;
    }
    pub fn is_authenticated(&self) -> bool {
        return self.is_authenticated;
    }
    pub fn get_user(&mut self) -> &mut Users {
        return &mut self.user;
    }
    pub fn get_itineary(&mut self) -> &mut Vec<Itinieary> {
        return &mut self.iteniary;
    }
}

async fn check_for_db(db_url: &str) -> bool {
    return sqlx::Postgres::database_exists(db_url)
        .await
        .unwrap_or(false);
}

pub async fn connect_to_db() -> Result<Pool<Postgres>, Error> {
    let c = CONFIG.get().unwrap();
    info!("db::connect_to_db():{:?}", c);
    let mut retries = 0;
    let mut connected = false;
    loop {
        if connected {
            break;
        }
        if retries >= c.get_envs().max_db_connection_retries {
            break;
        }
        info!(
            "db::connectToDB()::Attempting to connect to DB. Attempt number:{:?}",
            retries
        );
        retries += 1;
        connected = check_for_db(&c.get_envs().db_url).await;
        sleep(Duration::from_secs(
            c.get_envs().max_connections.try_into().unwrap(),
        ));
    }
    if !connected {
        return Err(anyhow::Error::msg(
            "db::connectToDB()::cannot connect to db exiting function",
        ));
    }
    let pool = match PgPoolOptions::new()
        .max_connections(c.get_envs().max_connections)
        .connect(&c.get_envs().db_url)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("error creating DB connection: {}", e);
            return Err(e.into());
        }
    };
    return Ok(pool);
}
