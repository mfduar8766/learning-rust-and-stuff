// use crate::utils::AsString;
// use chrono::offset::Utc;
// use chrono::DateTime;
// use once_cell::sync::Lazy;
// use serde_derive::{Deserialize, Serialize};
// use std::env;
// use std::fs::{self, File};
// use std::io::Error;
// use std::path::{Path, PathBuf};
// use std::time::SystemTime;

// const PATH: Lazy<PathBuf> = Lazy::new(|| {
//     let (_, file_format) = &get_time();
//     return create_dir(&create_file_name("rust-app", &file_format)).unwrap();
// });

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LogMessage<'a> {
//     value: &'a str,
//     message: &'a str,
// }

// impl<'a> LogMessage<'a> {
//     pub fn new(value: &'a str, message: &'a str) -> Self {
//         return Self { value, message };
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum LogLevel {
//     INFO,
//     WARNING,
//     ERROR,
//     FATAL,
// }

// impl AsString for LogLevel {
//     fn as_string(&self) -> &'static str {
//         match self {
//             &LogLevel::INFO => "INFO",
//             &LogLevel::WARNING => "WARNING",
//             &LogLevel::ERROR => "ERROR",
//             &LogLevel::FATAL => "FATAL",
//         }
//     }
// }

// #[derive(Debug, Serialize, Clone, Copy)]
// pub struct LoggerMessage<'a> {
//     message: &'a str,
//     value: &'a str,
//     time: &'a str,
//     level: &'a str,
//     service_name: &'a str,
// }

// #[allow(dead_code)]
// #[derive(Debug, Clone, Copy)]
// pub struct Logger<'a> {
//     service_name: &'a str,
//     logger_message: LoggerMessage<'a>,
// }

// impl<'a> Logger<'a> {
//     pub fn new(service_name: &'a str) -> Self {
//         // let (_, file_format) = &get_time();
//         // let path = create_dir(&create_file_name(service_name, &file_format)).unwrap();
//         return Self {
//             service_name,
//             // file_path: path,
//             logger_message: LoggerMessage::new(service_name),
//         };
//     }
//     pub fn log_infof(mut self, payload: LogMessage<'a>) -> Result<(), std::io::Error> {
//         self.logger_message
//             .set_log_data(LogLevel::INFO.as_string(), payload);
//         return self.logger_message.write_to_file();
//     }
//     pub fn log_info(mut self, payload: LogMessage<'a>) -> Result<(), Error> {
//         self.logger_message
//             .set_log_data(LogLevel::INFO.as_string(), payload);
//         return self.logger_message.write_to_file();
//     }
// }

// impl<'a> LoggerMessage<'a> {
//     fn new(service_name: &'a str) -> Self {
//         return Self {
//             service_name,
//             message: "",
//             value: "",
//             time: "",
//             level: "",
//         };
//     }
//     fn set_log_data(&mut self, level: &'a str, payload: LogMessage<'a>) {
//         let message = payload.message;
//         let value = payload.value;
//         // self.time = self.get_formated_time().to_string().as_str();
//         self.level = level;
//         self.message = message;
//         self.value = value;
//         self.print_message();
//     }
//     fn write_to_file(&self) -> Result<(), std::io::Error> {
//         let json_str = match serde_json::to_string(self) {
//             Ok(res) => res,
//             Err(e) => e.to_string(),
//         };
//         return match fs::write(PATH.as_path(), json_str) {
//             Ok(()) => Ok(()),
//             Err(e) => Err(e),
//         };
//     }
//     fn print_message(&mut self) {
//         if self.message.len() > 0 && self.value.len() > 0 {
//             println!("{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\",\"message\":\"{}\",\"value\":\"{}\"}}", self.time, self.level, self.service_name, self.message, self.value);
//         } else if self.message.len() == 0 {
//             println!(
//                 "{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\",\",\"value\":\"{}\"}}",
//                 self.time, self.level, self.service_name, self.value
//             );
//         } else if self.value.len() == 0 {
//             println!(
//                 "{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\",\"message\":\"{}\"}}",
//                 self.time, self.level, self.service_name, self.message
//             );
//         } else if self.message.len() == 0 && self.value.len() == 0 {
//             println!(
//                 "{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\"}}",
//                 self.time, self.level, self.service_name
//             );
//         }
//     }
// }

// fn create_file_name(service_name: &str, date: &str) -> String {
//     return format!("{}-{}.log", service_name, date);
// }

// fn create_dir(file_name: &str) -> Result<PathBuf, Error> {
//     let current_dir = env::current_dir();
//     match current_dir {
//         Ok(path_name) => {
//             let dir_name = &Path::join(&path_name, "Logs");
//             let full_path = Path::join(Path::new(dir_name), file_name);
//             if Path::exists(&full_path) {
//                 let file = File::create(full_path);
//                 match file {
//                     Ok(f) => Ok(Path::join(Path::new(dir_name), file_name)),
//                     Err(err) => panic!("{}", err),
//                 }
//             } else {
//                 let dir_name = &Path::join(&path_name, "Logs");
//                 let full_path = Path::join(Path::new(dir_name), file_name);
//                 match fs::create_dir_all(dir_name) {
//                     Ok(()) => match File::create(full_path) {
//                         Ok(f) => Ok(Path::join(Path::new(dir_name), file_name)),
//                         Err(err) => panic!("{}", err),
//                     },
//                     Err(err) => panic!("{}", err),
//                 }
//             }
//         }
//         Err(err) => panic!("{}", err),
//     }
// }

// fn get_time() -> (String, String) {
//     let system_time = SystemTime::now();
//     let datetime: DateTime<Utc> = system_time.into();
//     let log_format = datetime.format("%Y-%m-%d:%H:%M:%S.%MS").to_string();
//     let file_name_format = datetime.format("%Y-%m-%d").to_string();
//     return (log_format, file_name_format);
// }
