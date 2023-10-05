use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use time::OffsetDateTime;

use colored::Colorize;

const SUCCESS_LEVEL  : u8 = 5;
const INFO_LEVEL     : u8 = 4;
const ACTION_LEVEL   : u8 = 3;
const WARN_LEVEL     : u8 = 2;
const ERROR_LEVEL    : u8 = 1;

pub struct Logger {
  pub path: OsString,
  pub file: Option<File>,
  pub verbosity: u8,
}

impl Logger {
  pub fn new(path: OsString, verbosity: u8) -> Self {
    let file_wrapper = File::options()
      .create(true)
      .write(true)
      .append(true)
      .open(&path)
    ;

    let file = match file_wrapper {
      Ok(item) => Some(item),
      _ => None
    };

    Self {
      path: path,
      file: file,
      verbosity: verbosity,
    }
  }

  pub fn write(msg: String, file_opt: Option<&File>) {
    match file_opt{
      Some(mut file) => writeln!(file, "{}", msg).unwrap_or(()),
      _ => ()
    };
  }

  pub fn timestamp() -> Result<String, time::error::Error> {
    let format =  time::format_description::parse("[hour]:[minute]:[second]")?;
    let now_utc = OffsetDateTime::now_utc().format(&format)?;
    let output = format!("{} ", now_utc);
    Ok(output)
  }

  pub fn print(msg: &str, color: &str) {
    let msg_colored = format!("{}", msg.color(color));
    println!("{}", msg_colored);
  }

  pub fn error(&self, msg: String) {
    let color = "red";
    self.log(msg, color, ERROR_LEVEL);
  }

  pub fn warn(&self, msg: String) {
    let color = "yellow";
    self.log(msg, color, WARN_LEVEL);
  }

  pub fn action(&self, msg: String) {
    let color = "blue";
    self.log(msg, color, ACTION_LEVEL);
  }

  pub fn info(&self, msg: String) {
    let color = "cyan";
    self.log(msg, color, INFO_LEVEL);
  }

  pub fn success(&self, msg: String) {
    let color = "green";
    self.log(msg, color, SUCCESS_LEVEL);
  }

  pub fn log(&self, msg: String, color: &str, loglevel: u8) {
    if loglevel>self.verbosity {
      return
    };

    let prefix = Self::timestamp().unwrap_or("".to_string());
    let logline = format!("{}{}", prefix, msg);

    Self::print(&logline, color);
    Self::write(logline, self.file.as_ref());
    //std::thread::spawn(move || {
    //});
  }
}

#[macro_export]
macro_rules! error_msg {
    ($options: expr, $($arg:tt)*) => {
      $options.logger.error(format!($($arg)*))
    }
}
pub(crate) use error_msg;

#[macro_export]
macro_rules! info_msg {
    ($options: expr, $($arg:tt)*) => {
      $options.logger.info(format!($($arg)*))
    }
}
pub(crate) use info_msg;

#[macro_export]
macro_rules! warn_msg {
    ($options: expr, $($arg:tt)*) => {
      $options.logger.warn(format!($($arg)*))
    }
}
pub(crate) use warn_msg;

#[macro_export]
macro_rules! success_msg {
    ($options: expr, $($arg:tt)*) => {
      $options.logger.success(format!($($arg)*))
    }
}
pub(crate) use success_msg;

#[macro_export]
macro_rules! action_msg {
    ($options: expr, $($arg:tt)*) => {
      $options.logger.action(format!($($arg)*))
    }
}
pub(crate) use action_msg;
