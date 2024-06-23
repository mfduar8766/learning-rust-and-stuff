use anyhow::Error;
use chrono::{Datelike, TimeZone};
use chrono::{Local, Timelike, Utc};
use dotenv::dotenv;
use serde_derive::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, query, Executor};
use std::thread::sleep;
use std::time::Duration;
use std::{env, process};
use tokio;
use tracing::{error, info};
use tracing_subscriber;

#[derive(Debug)]
pub struct Envs {
    pub api_version: String,
    pub api_port: String,
    pub api_host: String,
    pub db_users_collection: String,
    pub db_itineary_collection: String,
    pub db_name: String,
    pub db_url: String,
    pub max_db_connection_retries: i32,
    pub use_db: bool,
    pub max_connections: u32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Users {
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

impl Envs {
    fn new() -> Self {
        let max_db_connection_env = env::var("MAX_DB_CONNECT_RETRIES")
            .unwrap_or(5.to_string())
            .parse::<i32>()
            .unwrap();
        let use_db_env = env::var("USE_DB").unwrap_or("false".to_string());
        let use_db = use_db_env.parse::<bool>().unwrap();
        return Self {
            api_version: env::var("API_VERSION").unwrap_or("v1".to_string()),
            api_port: env::var("API_PORT").unwrap_or("3000".to_string()),
            api_host: env::var("API_HOST").unwrap_or("127.0.0.1".to_string()),
            db_users_collection: env::var("DB_USERS_COLLECTION").unwrap_or("users".to_string()),
            db_itineary_collection: env::var("DB_ININEARY_COLLECTION")
                .unwrap_or("itineary".to_string()),
            db_name: env::var("DB_NAME").unwrap_or("travel".to_string()),
            db_url: env::var("DB_URL").unwrap_or("mongodb://localhost:27017".to_string()),
            max_db_connection_retries: max_db_connection_env,
            use_db,
            max_connections: env::var("MAX_CONNECTIONS")
                .unwrap_or(1.to_string())
                .parse::<u32>()
                .unwrap(),
        };
    }
}

impl Users {
    fn new() -> Self {
        let dt = Utc::now();
        // let dt = Local::now();
        // let naive_utc = dt.naive_utc();
        // let offset = dt.offset().clone();
        // let created_at = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        let created_at = Utc
            .with_ymd_and_hms(
                dt.year(),
                dt.month(),
                dt.day(),
                dt.hour(),
                dt.minute(),
                dt.second(),
            )
            .unwrap();
        return Self {
            email: String::from("test@tester12.com"),
            password: String::from("123"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            dob: String::from("06/07/1992"),
            id: 1,
            created_at,
            user_name: String::from("johnDoe"),
            last_login: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Itinieary {
    pub id: i32,
    pub destination: String,
    pub resource_id: String,
    pub departure: chrono::DateTime<chrono::Utc>,
    pub arrival: chrono::DateTime<chrono::Utc>,
    pub over_all_budget: String,
    pub user_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Itinieary {
    fn new(user_id: i32) -> Self {
        let dt = Utc::now();
        // let naive_utc = dt.to_utc();
        // let offset = dt.offset().clone();
        // let created_at =  //DateTime::<Utc>::from_naive_utc_and_offset(dt, dt.offset());
        let created_at = Utc
            .with_ymd_and_hms(
                dt.year(),
                dt.month(),
                dt.day(),
                dt.hour(),
                dt.minute(),
                dt.second(),
            )
            .unwrap();
        let departure = Utc.with_ymd_and_hms(dt.year(), 5, 22, 8, 0, 0).unwrap();
        let arrival = Utc.with_ymd_and_hms(dt.year(), 5, 23, 20, 0, 0).unwrap();
        return Self {
            id: 1,
            destination: String::from("Germany"),
            resource_id: String::from("Germany.png"),
            departure,
            arrival,
            over_all_budget: String::from("$100,000.00"),
            user_id,
            created_at,
        };
    }
}

async fn check_for_db(db_url: &str) -> bool {
    return sqlx::Postgres::database_exists(db_url)
        .await
        .unwrap_or(false);
}

async fn connect_and_populate_db() -> Result<(), Error> {
    let envs = Envs::new();
    info!("main::connectAndPopulateDB():{:?}", envs);
    let mut retries = 0;
    let mut connected = false;
    loop {
        if connected {
            break;
        }
        if retries >= envs.max_db_connection_retries {
            break;
        }
        info!(
            "main::connectAndPopulateDB()::Attempting to connect to DB. Attempt number:{:?}",
            retries
        );
        retries += 1;
        connected = check_for_db(&envs.db_url).await;
        sleep(Duration::from_secs(
            envs.max_db_connection_retries.try_into().unwrap(),
        ));
    }
    if !connected {
        return Err(anyhow::Error::msg("database does not exist"));
    }
    let pool = match PgPoolOptions::new()
        .max_connections(envs.max_connections)
        .connect(&envs.db_url)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("error creating DB connection: {}", e);
            return Err(e.into());
        }
    };
    let delete_users_table_query = format!("DROP TABLE IF EXISTS users CASCADE");
    let _ = match pool.execute(query(&delete_users_table_query)).await {
        Ok(_) => info!("main::connectAndPopulateDb()::successfully deleted user table"),
        Err(e) => {
            error!("error deleting users table: {}", e);
            return Err(e.into());
        }
    };
    let delete_itineary_table_query = format!("DROP TABLE IF EXISTS itineary CASCADE");
    let _ = match pool.execute(query(&delete_itineary_table_query)).await {
        Ok(_) => info!("main::connectAndPopulateDb()::successfully deleted itineary table"),
        Err(e) => {
            error!("error deleting itineary table: {}", e);
            return Err(e.into());
        }
    };
    let create_users_query = format!(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            first_name VARCHAR (255) NOT NULL,
            last_name VARCHAR (255) NOT NULL,
            user_name VARCHAR (50) UNIQUE NOT NULL,
            password VARCHAR (50) NOT NULL,
            email VARCHAR (255) UNIQUE NOT NULL,
            created_at DATE NOT NULL,
            last_login DATE,
            dob VARCHAR (12)
        );"
    );
    let _ = match pool.execute(query(&create_users_query)).await {
        Ok(_) => info!("main::connectAndPopulateDb()::successfully created user table"),
        Err(e) => {
            error!("error creating users table: {}", e);
            return Err(e.into());
        }
    };
    let create_itineary_query = format!(
        "CREATE TABLE IF NOT EXISTS itineary (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES users (id),
            destination VARCHAR (255),
            resource_id VARCHAR (255),
            departure DATE NOT NULL,
            arrival DATE NOT NULL,
            over_all_budget VARCHAR (50),
            created_at DATE NOT NULL
        );"
    );
    let _ = match pool.execute(query(&create_itineary_query)).await {
        Ok(_) => info!("main::connectAndPopulateDb()::successfully created itineary table"),
        Err(e) => {
            error!("error creating users table: {}", e);
            return Err(e.into());
        }
    };
    let user_instance = Users::new();
    let itineary_instance = Itinieary::new(user_instance.id);
    let _ = match sqlx::query(
            "INSERT INTO users (id, first_name, last_name, user_name, password, email, created_at, last_login, dob)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning id")
            .bind(user_instance.id)
            .bind(user_instance.first_name)
            .bind(user_instance.last_name)
            .bind(user_instance.user_name)
            .bind(user_instance.password)
            .bind(user_instance.email)
            .bind(user_instance.created_at)
            .bind(user_instance.last_login)
            .bind(user_instance.dob)
            .execute(&pool).await {
                Ok(_) => info!("main::connectAndPopulateDb()::successfully populated user table"),
                Err(e) => {
                    error!("error populating users table: {}", e);
                    return Err(e.into());
                }
            };
    let _ = match sqlx::query(
                "INSERT INTO itineary (id, user_id, destination, resource_id, departure, arrival, over_all_budget, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8) returning id")
                .bind(itineary_instance.id)
                .bind(itineary_instance.user_id)
                .bind(itineary_instance.destination)
                .bind(itineary_instance.resource_id)
                .bind(itineary_instance.departure)
                .bind(itineary_instance.arrival)
                .bind(itineary_instance.over_all_budget)
                .bind(itineary_instance.created_at)
                .execute(&pool).await {
                    Ok(_) => info!("main::connectAndPopulateDb()::successfully populated itineary table"),
                    Err(e) => {
                        error!("error populating itineary table: {}", e);
                        return Err(e.into());
                    }
                };
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let _ = match connect_and_populate_db().await {
        Ok(_) => {
            info!("main()::successfully created and populated tables");
            process::exit(0);
        }
        Err(e) => {
            error!("main()::error exitig program: {}", e);
            process::exit(1);
        }
    };
}
