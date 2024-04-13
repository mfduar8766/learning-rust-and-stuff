use crate::utils::AsString;
use chrono::offset::Utc;
use chrono::DateTime;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::path::Path;
use std::sync::Mutex;
use std::time::SystemTime;

lazy_static! {
    static ref TASKS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMessage {
    value: String,
    message: String,
}

impl LogMessage {
    pub fn new(value: &str, message: &str) -> Self {
        return Self {
            value: value.to_string(),
            message: message.to_string(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogLevel {
    INFO,
    WARNING,
    ERROR,
    FATAL,
}

impl AsString for LogLevel {
    fn as_string(&self) -> &'static str {
        match self {
            &LogLevel::INFO => "INFO",
            &LogLevel::WARNING => "WARNING",
            &LogLevel::ERROR => "ERROR",
            &LogLevel::FATAL => "FATAL",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerMessage {
    message: String,
    value: String,
    time: String,
    level: String,
    service_name: String,
}

#[derive(Debug)]
pub struct Logger {
    log_message: LoggerMessage,
    service_name: String,
    file: File,
}

impl LoggerMessage {
    pub fn new(service_name: String) -> Self {
        return Self {
            service_name,
            message: String::new(),
            value: String::new(),
            time: String::new(),
            level: String::new(),
        };
    }
    fn write_to_file(&self, file: File) -> Result<(), std::io::Error> {
        let mut writer = BufWriter::new(file);
        match serde_json::to_writer(&mut writer, self) {
            Ok(()) => {
                writer
                    .flush()
                    .expect("Error writing to file and flushing contents");
                return Ok(());
            }
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

impl Logger {
    pub fn new(service_name: &str) -> Self {
        let (_, file_format) = &get_time();
        return Self {
            log_message: LoggerMessage::new(service_name.to_string()),
            service_name: service_name.to_string(),
            file: create_dir(&create_file_name(service_name, &file_format)).unwrap(),
        };
    }

    pub fn log_infof(mut self, payload: LogMessage) -> Result<(), std::io::Error> {
        self.set_log_data(LogLevel::INFO.as_string(), payload);
        return self.log_message.write_to_file(self.file);
    }

    pub fn log_info(mut self, payload: LogMessage) -> Result<(), Error> {
        self.set_log_data(LogLevel::INFO.as_string(), payload);
        return self.log_message.write_to_file(self.file);
    }

    fn set_log_data(&mut self, level: &str, payload: LogMessage) {
        let (log_format, _) = get_time();
        let message = &payload.message;
        let value = &payload.value;
        let log_time = &log_format;
        self.log_message.time = log_time.to_string();
        self.log_message.level = level.to_string();
        self.log_message.message = message.to_string();
        self.log_message.value = value.to_string();
        self.print_message(&log_format, level, message, value);
    }

    fn print_message(&mut self, log_format: &str, level: &str, message: &str, value: &str) {
        if message.len() > 0 && value.len() > 0 {
            println!("{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\",\"message\":\"{}\",\"value\":\"{}\"}}", log_format, level, self.service_name, message, value);
        } else if message.len() == 0 {
            println!(
                "{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\",\",\"value\":\"{}\"}}",
                log_format, level, self.service_name, value
            );
        } else if value.len() == 0 {
            println!(
                "{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\",\"message\":\"{}\"}}",
                log_format, level, self.service_name, message
            );
        } else if message.len() == 0 && value.len() == 0 {
            println!(
                "{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\"}}",
                log_format, level, self.service_name
            );
        }
    }
}

fn create_file_name(service_name: &str, date: &str) -> String {
    return format!("{}-{}.log", service_name, date);
}

fn create_dir(file_name: &str) -> Result<File, Error> {
    let current_dir = env::current_dir();
    match current_dir {
        Ok(path_name) => {
            let dir_name = &Path::join(&path_name, "Logs");
            let full_path = Path::join(Path::new(dir_name), file_name);
            if Path::exists(&full_path) {
                let file = File::create(full_path);
                match file {
                    Ok(f) => Ok(f),
                    Err(err) => panic!("{}", err),
                }
            } else {
                match fs::create_dir_all(dir_name) {
                    Ok(()) => match File::create(Path::join(Path::new(dir_name), file_name)) {
                        Ok(f) => Ok(f),
                        Err(err) => panic!("{}", err),
                    },
                    Err(err) => panic!("{}", err),
                }
            }
        }
        Err(err) => panic!("{}", err),
    }
}

fn get_time() -> (String, String) {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    let log_format = datetime.format("%Y-%m-%d:%H:%M:%S.%MS").to_string();
    let file_name_format = datetime.format("%Y-%m-%d").to_string();
    return (log_format, file_name_format);
}
