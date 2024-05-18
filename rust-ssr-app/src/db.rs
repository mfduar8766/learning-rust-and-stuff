use crate::CONFIG;
use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions};
use sqlx::{Pool, Postgres};
use tracing::{error, info, warn};

#[derive(Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Users {
    id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub last_login: Option<chrono::DateTime<chrono::Local>>,
    pub user_name: String,
    password: String,
}

#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Itinieary {
    pub id: i32,
    pub destination: String,
    pub resource_id: String,
    pub departure: chrono::DateTime<chrono::Local>,
    pub arrival: chrono::DateTime<chrono::Local>,
    pub over_all_budget: String,
    pub user_id: i32,
    pub created_at: chrono::DateTime<chrono::Local>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Db {
    pub user: Users,
    pub iteniary: Vec<Itinieary>,
    pub db: Pool<Postgres>,
    is_authenticated: bool,
}

impl Db {
    pub fn new(pg_db: Pool<Postgres>) -> Self {
        return Self {
            db: pg_db,
            is_authenticated: false,
            user: Users::default(),
            iteniary: vec![Itinieary::default()],
        };
    }
    pub async fn authenticate(&mut self, email: &str, password: &str) -> bool {
        if email.len() == 0 || password.len() == 0 {
            return false;
        }
        let check_user_name_and_password = format!(
            "SELECT DISTINCT FROM {} WHERE email = ? AND password = ?",
            CONFIG.lock().unwrap().get_envs().db_users
        );
        match sqlx::query_as::<_, Users>(&check_user_name_and_password)
            .bind(email)
            .bind(password)
            .fetch_one(&self.db)
            .await
        {
            Ok(users) => {
                info!("db::authenticate()::user:{:?}", users);
                self.user = users;
                let get_itinearies = format!(
                    "SELECT DISTINCT FROM {} WHERE user_id = ?",
                    CONFIG.lock().unwrap().get_envs().db_itineary
                );
                self.is_authenticated = true;
                match sqlx::query_as::<_, Itinieary>(&get_itinearies)
                    .bind(self.user.id)
                    .fetch_all(&self.db)
                    .await
                {
                    Ok(itineary) => {
                        self.iteniary = itineary;
                        return true;
                    }
                    Err(e) => {
                        warn!("db::authenticate()::error getting itinearies:{}", e);
                        return false;
                    }
                }
            }
            Err(e) => {
                warn!("db::authenticate()::error:{}", e);
                return false;
            }
        }
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
}

pub async fn connect_to_db() -> Result<Db, Error> {
    if !sqlx::Postgres::database_exists(&CONFIG.lock().unwrap().get_envs().db_url)
        .await
        .unwrap_or(false)
    {
        return Err(anyhow::Error::msg("database does not exist"));
    }
    let pool = match PgPoolOptions::new()
        .max_connections(CONFIG.lock().unwrap().get_envs().max_connections)
        .connect(&CONFIG.lock().unwrap().get_envs().db_url)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("error creating DB connection: {}", e);
            return Err(e.into());
        }
    };
    return Ok(Db::new(pool));
}
