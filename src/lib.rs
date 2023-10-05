use std::ffi::OsString;

#[cfg(test)]
mod tests;

mod settings;
mod entries;

pub mod cases;
pub mod logging;

use crate::logging::Logger;

pub struct Options {
    pub verbosity: u8,
    pub nocheckcontent: bool,
    pub onlypath: bool,
    pub regular_info: bool,
    pub logpath: OsString,
    pub logger: Logger
}

impl Options {
  pub fn new( verbosity: u8, nocheckcontent: bool, onlypath: bool, logpath: OsString, regular_info: bool) -> Self {
    let logger = Logger::new(logpath.clone(), verbosity);
    Self {
      verbosity:      verbosity,
      nocheckcontent: nocheckcontent,
      onlypath:       onlypath,
      logpath:        logpath,
      regular_info:   regular_info,
      logger:         logger,
    }
  }
}
