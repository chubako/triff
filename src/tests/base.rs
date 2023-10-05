use std::ffi::OsString;
use std::io::ErrorKind;

use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::fs;

use super::Options;
use super::entries::Entry;

const ASSETS_BASE:&str = "src/tests/assets/test_00_base";

struct FileUnreadable {
  path: OsString,
  mode: u32,
}

impl FileUnreadable {
  pub fn new(path: OsString) -> std::io::Result<Self> {
    let meta = fs::symlink_metadata(&path)?;
    let mode = meta.mode();

    let item = Self {
      path: path,
      mode: mode,
    };

    //remove read permissions
    item.set_mode(0o000)?;

    Ok(item)
  }

  pub fn restore_mode(&self) -> std::io::Result<()> {
    self.set_mode(self.mode)?;
    Ok(())
  }

  pub fn set_mode(&self, mode: u32) -> std::io::Result<()> {
    let meta = fs::symlink_metadata(&self.path)?;
    let mut permissions = meta.permissions();
    permissions.set_mode(mode);
    fs::set_permissions(&self.path, permissions)?;
    Ok(())
  }
}

impl Drop for FileUnreadable {
  fn drop(&mut self) {
    self.restore_mode().unwrap_or(());
  }
}

// ----------
// Tests
// ----------

#[test]
pub fn test1_existence() -> std::io::Result<()> {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path_doesnt_exist = OsString::from(vec![ASSETS_BASE, "a/file0"].join("/"));

  match Entry::new(&path_doesnt_exist, &options) {
    Ok(_) => panic!("Path doesn't exist, but Entry created"),
    Err(error) => match error.kind() {
      ErrorKind::NotFound=> assert!(true),
      _ => panic!("Entry could not be created by unknow reason")
    }
  };

  Ok(())
}

#[test]
pub fn test2_file_readable() -> std::io::Result<()> {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path_unreadable = OsString::from(vec![ASSETS_BASE, "a/file2"].join("/"));
  let file_unreadable = FileUnreadable::new(path_unreadable)?;

  match Entry::new(&file_unreadable.path, &options) {
    Ok(_) => assert!(false, "Path is unredable, but Entry created"),
    Err(error) => match error.kind() {
      ErrorKind::PermissionDenied => assert!(true),
      _ => assert!(false, "Entry could not be created by unknow reason: {:?}", error)
    }
  };

  let path_readable = OsString::from(vec![ASSETS_BASE, "a/file1"].join("/"));

  match Entry::new(&path_readable, &options) {
    Ok(_) => assert!(true), 
    Err(_) => assert!(false, "Path readable, but can't create entry"),
  };

  Ok(())
}
