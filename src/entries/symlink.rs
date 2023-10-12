use std::path::PathBuf;
use std::fs;
use std::ffi::OsString;
use std::io::{Error, ErrorKind};

use crate::settings::*;
use crate::logging::{error_msg, success_msg, action_msg};
use crate::Options;
use crate::entries::{Entry};

pub struct EntrySymlink<'a> {
  options: &'a Options,
  path: &'a OsString,
}

impl<'a> EntrySymlink<'a> {
  pub fn from(entry: Entry<'a>) -> Self {
    Self {
      options:             entry.options,
      path:                entry.path,
    }
  }

  pub fn link(&self) -> std::io::Result<PathBuf> {
    match fs::read_link(self.path) {
      Ok(alink) => Ok(alink),
      Err(_) => {
        error_msg!(self.options, "{ERROR_SYMLINK_READ} {:?} [can't read symlink]", self.path);
        action_msg!(self.options, "{NOOP}; ls -d -l {:?}", self.path);
        Err(Error::new(ErrorKind::InvalidData, ""))
      }
    }
  }

  pub fn compare(&self, other: &Self) -> std::io::Result<()> {
    if self.link()? == other.link()? {
      success_msg!(self.options, "{SUCCESS_SYMLINK} {:?}", other.path);
      action_msg!(self.options, "{SUCCESS_SYMLINK}; rm    {:?}", self.path);
      return Ok(())
    } else {
      error_msg!(self.options, "{ERROR_SYMLINKS_DIFFERENT} {:?} [symlinks different]", self.path);
      action_msg!(self.options, "{NOOP}; ls -d -l {:?} {:?}", self.path, other.path);
      return Err(Error::new(ErrorKind::InvalidData, ""))
    }
  }
}
