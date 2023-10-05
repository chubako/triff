use std::ffi::OsString;
use std::fs::Metadata;

use crate::settings::*;
use crate::logging::{error_msg, warn_msg, success_msg, action_msg};
use crate::Options;
use crate::cases::{check_inode, check_content,};
use crate::entries::{Entry};

pub struct EntryFile<'a> {
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


impl<'a> EntryFile<'a> {
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

  pub fn compare_inums(&self, other: &Self) -> std::io::Result<bool> {
    match check_inode(self.path, other.path) {
      Ok(_) => {
        success_msg!(self.options, "{SUCCESS_INODE} {} [same inode]", self.target_str);
        action_msg!(self.options, "{SUCCESS_INODE}; rm    '{}'", self.target_str);
        Ok(true)
      },
      Err(_) => Ok(false),
    }
  }

  pub fn compare_sizes(&self, other: &Self) -> bool {
    if self.size == other.size {
      return true
    }
    let diff_size = (self.size as i64) - (other.size as i64);

    warn_msg!(self.options, "{WARN_SIZE} {} [size: {} != {} ({})] ", self.target_str, self.size, other.size, diff_size);
    action_msg!(self.options, "{WARN_SIZE}; ls -l -s --block-size=1 '{}' '{}'", self.target_str, other.target_str);

    return false
  }

  pub fn compare_real_sizes(&self, other: &Self) -> bool {
    if self.real_size == other.real_size {
      return true
    }

    if (self.real_size  > other.real_size + self.real_size_tolerance) ||
       (other.real_size > self.real_size  + other.real_size_tolerance)
       {
        let diff_real_size = (self.real_size as i64) - (other.real_size as i64);
        warn_msg!(self.options, "{WARN_REAL_SIZE} {} [real size: {} != {} ({})] ", self.target_str, self.real_size, other.real_size, diff_real_size);
        action_msg!(self.options, "{WARN_REAL_SIZE}; ls -l -s --block-size=1 '{}' '{}'", self.target_str, other.target_str);
        return false
    }

    return true
  }

  pub fn compare_contents(&self, other: &Self) -> std::io::Result<SuccessKind> {
    if self.options.nocheckcontent {
      success_msg!(self.options, "{SUCCESS_SKIP_CONTENT} {} [skipping content]", self.target_str);
      action_msg!(self.options, "{SUCCESS_SKIP_CONTENT}; rm    '{}'", self.target_str);
      return Ok(SuccessKind::SkipContent)
    }

    match check_content(self.path, other.path) {
      Ok(_) => {
        success_msg!(self.options, "{SUCCESS_CONTENT} {}", self.target_str);
        action_msg!(self.options, "{SUCCESS_CONTENT}; rm    '{}'", self.target_str);
        return Ok(SuccessKind::SameContent)
      },
      Err(error) => {
        error_msg!(self.options, "{ERROR_CONTENT} {} [content]", self.target_str);
        action_msg!(self.options, "{NOOP}; ls -d -l '{}' '{}'", self.target_str, other.target_str);
        return Err(error)
      }
    }
  }

  pub fn compare(&self, other: &Self) -> std::io::Result<SuccessKind> {
    match self.compare_inums(other) {
      Ok(true) => return Ok(SuccessKind::SameInums),
      _ => ()
    }

    self.compare_sizes(other);
    self.compare_real_sizes(other);
    self.compare_contents(other)?;

    Ok(SuccessKind::Other)
  }
}
