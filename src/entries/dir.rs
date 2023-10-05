use std::path::Path;
use std::fs;
use std::ffi::OsString;
use std::io::{Error, ErrorKind};
use std::fs::Metadata;
use std::collections::HashSet;

use crate::settings::*;
use crate::logging::{error_msg, warn_msg, success_msg, action_msg, info_msg};
use crate::Options;
use crate::entries::{Entry};
use crate::cases::compare::compare_entries;

pub struct EntryDir<'a> {
  pub options: &'a Options,
  pub path: &'a OsString,
  pub target_str: String,
  pub meta: Metadata,
  pub mode: u32,
  pub file_type: char,
  pub size: u64,
  pub blocks: u64,
  pub real_size: u64,
  pub blksize: u64,
  pub real_size_tolerance: u64,
}


impl<'a> EntryDir<'a> {
  pub fn from(entry: Entry<'a>) -> Self {
    Self {
      options:             entry.options,
      target_str:          entry.target_str,
      path:                entry.path,
      meta:                entry.meta,
      mode:                entry.mode,
      file_type:           entry.file_type,
      size:                entry.size,
      blocks:              entry.blocks,
      real_size:           entry.real_size,
      blksize:             entry.blksize,
      real_size_tolerance: entry.real_size_tolerance,
    }
  }

  pub fn compare(&self, other: &Self) -> std::io::Result<()> {
    info_msg!(self.options, "+ {}", self.target_str);

    let entries1 : HashSet<OsString> = fs::read_dir(self.path).unwrap()
      .map(|r| r.unwrap().file_name())
      .collect();

    let entries2 : HashSet<OsString> = fs::read_dir(other.path).unwrap()
      .map(|r| r.unwrap().file_name())
      .collect();

    let excluded = entries2.difference(&entries1);

    for entry in excluded {
      let entry_str = entry.as_os_str().to_string_lossy();
      warn_msg!(self.options, "{WARN_NOT_IN} {:?} [outlier: {}]", self.target_str, entry_str);
      action_msg!(self.options, "{NOOP}; ls -l '{:?}' '{:?}'", self.target_str, other.target_str);
    }

    //This is not strictly necessary, but gives better outputs for users (walking directories in
    //alphabetic order). Some performance penalty.
    let mut entries = entries1.into_iter().collect::<Vec<_>>();
    entries.sort();

    let mut same_dir = true;
    for entry in entries {
      let new_path1  = Path::new(self.path).join(&entry).as_os_str().to_os_string();
      let new_path2  = Path::new(other.path).join(&entry).as_os_str().to_os_string();

      let outcome  = (|| -> Result<(), Error> {
        let entry1 = Entry::new(&new_path1, &self.options)?;
        let entry2 = Entry::new(&new_path2, &self.options)?;
        compare_entries(entry1, entry2)?;
        Ok(())
      })();

      //If an entry is different keep comparing the others, to catch as many differences as
      //possible in one run
      match outcome {
        Err(_) => same_dir = false,
        Ok(_)  => (),
      }
    }

    match same_dir {
      true => {
        success_msg!(self.options, "{SUCCESS_DIR} {:?} [{:?}]", other.target_str, self.target_str);
        action_msg!(self.options, "{SUCCESS_DIR}; rmdir '{}'", &self.target_str);
        return Ok(())
      },
      false => {
        error_msg!(self.options, "{ERROR_DIRECTORY} {:?} [directory different]",self.target_str);
        action_msg!(self.options, "{NOOP}; ls -l '{:?}' '{:?}'", self.target_str, other.target_str);
        return Err(Error::new(ErrorKind::InvalidData, ""));
      }
    }
  }
}
