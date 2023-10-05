use std::fs;
use std::ffi::OsString;
use std::io::{Error, ErrorKind};

use std::os::unix::fs::MetadataExt;
use std::fs::Metadata;

use permissions::is_readable;

use crate::settings::*;
use crate::logging::{error_msg, warn_msg, success_msg, action_msg};
use crate::Options;

use crate::cases::{check_permissions, check_file_types,};

pub struct Entry<'a> {
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

impl<'a> Entry<'a> {
  fn path_to_str(path: &'a OsString) -> String {
    let output = path.to_string_lossy();
    let output = output.replace("'", "'\\''");
    return output
  }

  fn file_type(mode: u32) -> char {
    let file_type_bin = mode & libc::S_IFMT;
    //{file type constant}: {file type}
    //S_IFDIR:  directory file.
    //S_IFCHR:  character-oriented device file.
    //S_IFBLK:  block-oriented device file.
    //S_IFREG:  regular file.
    //S_IFLNK:  symbolic link.
    //S_IFSOCK: socket.
    //S_IFIFO:  FIFO or pipe.
    match file_type_bin {
      libc::S_IFDIR  => 'd',
      libc::S_IFCHR  => 'c',
      libc::S_IFBLK  => 'b',
      libc::S_IFREG  => '-',
      libc::S_IFLNK  => 'l',
      libc::S_IFSOCK => 's',
      libc::S_IFIFO  => 'f',
      _ => '?',
    }
  }

  fn fmt_permissions(mode: u32) -> String {
    let ft = Self::file_type(mode);

    let own_read  = if mode & libc::S_IRUSR>0 {"r"} else {"-"};
    let grp_read  = if mode & libc::S_IRGRP>0 {"r"} else {"-"};
    let oth_read  = if mode & libc::S_IROTH>0 {"r"} else {"-"};

    let own_write = if mode & libc::S_IWUSR>0 {"w"} else {"-"};
    let grp_write = if mode & libc::S_IWGRP>0 {"w"} else {"-"};
    let oth_write = if mode & libc::S_IWOTH>0 {"w"} else {"-"};

    let own_exec = if mode & libc::S_IXUSR>0 {"x"} else {"-"};
    let grp_exec = if mode & libc::S_IXGRP>0 {"x"} else {"-"};
    let oth_exec = if mode & libc::S_IXOTH>0 {"x"} else {"-"};

    let output = format!("{}{}{}{}{}{}{}{}{}{}", ft, own_read, own_write, own_exec, grp_read, grp_write, grp_exec, oth_read, oth_write, oth_exec);
    return output
  }

  pub fn new(path_osstr: &'a OsString, options: &'a Options) -> std::io::Result<Self> {
    let target_str = Self::path_to_str(path_osstr);

    let meta = match fs::symlink_metadata(path_osstr) {
        Ok(meta_data) => meta_data,
        Err(code) => match code.kind() {
          ErrorKind::NotFound => {
            error_msg!(options,"{ERROR_NOT_FOUND} '{target_str}' [not found]");
            action_msg!(options, "{NOOP}; ls -d -l '{}'", target_str);
            return Err(code)
          },
          _ => {
            error_msg!(options,"{ERROR_CANT_READ_METADATA} {target_str} [can't read metadata on src ({code}) ]");
            action_msg!(options, "{NOOP}; ls -d -l '{}'", target_str);
            return Err(Error::new(ErrorKind::Other, ""));
          }
        }
    };

    let mode = meta.mode();
    let file_type = Self::file_type(mode);
    let size = meta.len();
    let blocks = meta.blocks();
    let real_size = blocks*META_BLOCK_SIZE;

    let blksize = meta.blksize();
    let real_size_tolerance = blksize;

    let item = Self {
      options: options,
      target_str: target_str,
      path: &path_osstr,
      meta: meta,
      mode: mode,
      file_type: file_type,
      size: size,
      blocks: blocks,
      real_size: real_size,
      blksize: blksize,
      real_size_tolerance: real_size_tolerance,
    };

    item.validate()?;

    Ok(item)
  }

  pub fn validate(&self) -> std::io::Result<()> {
    self.check_readable()?;
    Ok(())
  }

  pub fn check_readable(&self) -> std::io::Result<()> {
    let can_read = if self.meta.is_symlink() {
      Ok(true)
    } else {
      is_readable(self.path)
    };

    match can_read {
      Ok(boolean) => match boolean {
        true => (),
        false => {
          error_msg!(self.options,"{ERROR_UNREADABLE} {} [unreadable src]", self.target_str);
          action_msg!(self.options, "{NOOP}; ls -d -l '{}'", self.target_str);
          return Err(Error::new(ErrorKind::PermissionDenied, ""));
        }
      },
      Err(_) => {
        error_msg!(self.options,"{ERROR_UNACCESSIBLE} {} [unaccessible src]", self.target_str);
        action_msg!(self.options, "{NOOP}; ls -d -l '{}'", self.target_str);
        return Err(Error::new(ErrorKind::Other, ""));
      }
    };

    Ok(())
  }

  pub fn compare_paths(&self, other: &Self) -> std::io::Result<bool> {
    if self.path==other.path {
      return Ok(true)
    } else {
      error_msg!(self.options,"{ERROR_ONLY_PATH} {:?} [different_paths: {:?} {:?}]", &self.target_str, self.path, other.path);
      action_msg!(self.options, "{NOOP}; ls -d -l '{:?}' '{:?}'", &self.target_str, &other.target_str);
      return Err(Error::new(ErrorKind::Other, ""))
    }
  }

  pub fn compare_file_types(&self, other: &Self) -> std::io::Result<()> {
      if check_file_types(self.mode, other.mode) {
        Ok(())
      } else {
        error_msg!(self.options,"{ERROR_FILE_TYPE} {} [different_types: {} {}]", &self.target_str, &self.file_type, &self.file_type);
        action_msg!(self.options, "{NOOP}; ls -d -l '{}' '{}'", &self.target_str, &other.target_str);
        Err(Error::new(ErrorKind::Other, ""))
      }
  }

  pub fn compare_permissions(&self, other: &Self) -> std::io::Result<()> {
    if check_permissions(self.mode, other.mode) {
      Ok(())
    } else {
      let mode1_formatted = Self::fmt_permissions(self.mode);
      let mode2_formatted = Self::fmt_permissions(other.mode);
      warn_msg!(self.options, "{WARN_PERMISSION} {} [permissions: {} {}]", &self.target_str, mode1_formatted, mode2_formatted);
      action_msg!(self.options, "{NOOP}; ls -d -l '{}' '{}'", &self.target_str, &other.target_str);
      Ok(())
    }
  }

  pub fn compare_owners(&self, other: &Self) -> std::io::Result<()> {
    if self.meta.uid()==other.meta.uid() {
      Ok(())
    } else {
      warn_msg!(self.options, "{WARN_OWNERS} {} [permissions: {} {}]", &self.target_str, self.meta.uid(), other.meta.uid());
      action_msg!(self.options, "{NOOP}; ls -d -l '{}' '{}'", &self.target_str, &other.target_str);
      Ok(())
    }
  }

  pub fn compare_groups(&self, other: &Self) -> std::io::Result<()> {
    if self.meta.gid()==other.meta.gid() {
      Ok(())
    } else {
      warn_msg!(self.options, "{WARN_GROUPS} {} [permissions: {} {}]", &self.target_str, self.meta.gid(), other.meta.gid());
      action_msg!(self.options, "{NOOP}; ls -d -l '{}' '{}'", &self.target_str, &other.target_str);
      Ok(())
    }
  }

  pub fn compare(&self, other: &Self) -> std::io::Result<bool> {
//    self.compare_paths(other)?;
    if self.options.onlypath {
      success_msg!(self.options, "{SUCCESS_ONLY_PATH} {} [only path]", self.target_str);
      action_msg!(self.options, "{SUCCESS_ONLY_PATH}; rm    '{}'", self.target_str);
      return Ok(true)
    }

    self.check_readable()?;
    other.check_readable()?;
    self.compare_file_types(other)?;
    self.compare_permissions(other)?;
    self.compare_owners(other)?;
    self.compare_groups(other)?;

    Ok(false)
  }
}
