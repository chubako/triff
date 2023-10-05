use std::ffi::OsString;

use crate::settings::*;
use crate::logging::{warn_msg, action_msg, info_msg};
use crate::Options;
use crate::entries::{Entry};

pub struct EntryNonregular<'a> {
  pub options: &'a Options,
  pub path: &'a OsString,
  pub target_str: String,
}

impl<'a> EntryNonregular<'a> {
  pub fn from(entry: Entry<'a>) -> Self {
    Self {
      options:             entry.options,
      path:                entry.path,
      target_str:          entry.target_str,
    }
  }

  pub fn compare(&self, _other: &Self) -> std::io::Result<SuccessKind> {
    if self.options.regular_info {
      info_msg!(self.options, "{WARN_REGULAR_FILE} {} [not regular file]", &self.target_str);
    } else {
      warn_msg!(self.options, "{WARN_REGULAR_FILE} {} [not regular file]", &self.target_str);
    }
    action_msg!(self.options, "{WARN_REGULAR_FILE}; rmnrf '{}'", self.target_str);

    return Ok(SuccessKind::NonRegular)
  }
}
