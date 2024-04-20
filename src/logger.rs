// use crate::utils::AsString;
// use chrono::offset::Utc;
// use chrono::DateTime;
// use once_cell::sync::Lazy;
// use serde_derive::{Deserialize, Serialize};
// use std::env;
// use std::fs::{self, File};
// use std::io::{BufWriter, Error, ErrorKind, Write};
// use std::path::Path;
// use std::time::SystemTime;

// const FILE: Lazy<File> = Lazy::new(|| {
//     let (_, file_format) = get_time();
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

// #[derive(Debug, Serialize)]
// pub struct LoggerMessage<'a> {
//     message: &'a str,
//     value: &'a str,
//     time: String,
//     level: &'a str,
//     service_name: &'a str,
// }

// impl<'a> LoggerMessage<'a> {
//     pub fn new(service_name: &'a str) -> Self {
//         return Self {
//             service_name: &service_name,
//             message: "",
//             value: "",
//             time: String::new(),
//             level: "",
//         };
//     }
//     fn write_to_file(&self) -> Result<(), std::io::Error> {
//         let binding = &mut FILE;
//         let mut writer = BufWriter::new(binding.by_ref());
//         match serde_json::to_writer(&mut writer, self) {
//             Ok(()) => {
//                 writer
//                     .flush()
//                     .expect("Error writing to file and flushing contents");
//                 return Ok(());
//             }
//             Err(e) => Err(Error::new(ErrorKind::Other, e)),
//         }
//     }
//     pub fn log_infof(mut self, payload: LogMessage<'a>) -> Result<(), std::io::Error> {
//         self.set_log_data(LogLevel::INFO.as_string(), payload);
//         return self.write_to_file();
//     }

//     pub fn log_info(mut self, payload: LogMessage<'a>) -> Result<(), Error> {
//         self.set_log_data(LogLevel::INFO.as_string(), payload);
//         return self.write_to_file();
//     }

//     fn set_log_data(&mut self, level: &'a str, payload: LogMessage<'a>) {
//         let (log_format, _) = get_time();
//         let message = payload.message;
//         let value = payload.value;
//         let format = &log_format;
//         self.time = format.to_string();
//         self.level = level;
//         self.message = &message;
//         self.value = &value;
//         self.print_message(&log_format, level, &message, &value);
//     }

//     fn print_message(&mut self, log_format: &str, level: &str, message: &str, value: &str) {
//         if message.len() > 0 && value.len() > 0 {
//             println!("{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\",\"message\":\"{}\",\"value\":\"{}\"}}", log_format, level, self.service_name, message, value);
//         } else if message.len() == 0 {
//             println!(
//                 "{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\",\",\"value\":\"{}\"}}",
//                 log_format, level, self.service_name, value
//             );
//         } else if value.len() == 0 {
//             println!(
//                 "{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\",\"message\":\"{}\"}}",
//                 log_format, level, self.service_name, message
//             );
//         } else if message.len() == 0 && value.len() == 0 {
//             println!(
//                 "{{\"time\": \"{}\",\"level\":\"{}\",\"service\":\"{}\"}}",
//                 log_format, level, self.service_name
//             );
//         }
//     }
// }

// fn create_file_name(service_name: &str, date: &str) -> String {
//     return format!("{}-{}.log", service_name, date);
// }

// fn create_dir(file_name: &str) -> Result<File, Error> {
//     let current_dir = env::current_dir();
//     match current_dir {
//         Ok(path_name) => {
//             let dir_name = &Path::join(&path_name, "Logs");
//             let full_path = Path::join(Path::new(dir_name), file_name);
//             if Path::exists(&full_path) {
//                 let file = File::create(full_path);
//                 match file {
//                     Ok(f) => Ok(f),
//                     Err(err) => panic!("{}", err),
//                 }
//             } else {
//                 match fs::create_dir_all(dir_name) {
//                     Ok(()) => match File::create(Path::join(Path::new(dir_name), file_name)) {
//                         Ok(f) => Ok(f),
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

// // fn get_log_format<'a>(mut log_format: &'a str) -> &'a str {
// //     let system_time = SystemTime::now();
// //     let datetime: DateTime<Utc> = system_time.into();
// //     let binding = &datetime.format(&log_format).to_string();
// //     log_format = binding;
// //     return log_format;
// // }
