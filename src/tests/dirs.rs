use std::ffi::OsString;

use super::Options;
use super::entries::{Entry, EntryDir};

const ASSETS_BASE:&str = "src/tests/assets/test_04_dirs";

#[test]
pub fn test_different_directories() -> std::io::Result<()> {
  let options =  Options::new(
    9,
    false,
    false,
    OsString::new(),
    false
  );

  let path1 = OsString::from(vec![ASSETS_BASE, "a"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let dir1 = EntryDir::from(entry1);
  let dir2 = EntryDir::from(entry2);
  match dir1.compare(&dir2) {
    Ok(()) => panic!("different directories compared, but reported as equal"),
    _  => assert!(true),
  }

  Ok(())
}

#[test]
pub fn test_different_directories2() -> std::io::Result<()> {
  let options =  Options::new(
    9,
    false,
    false,
    OsString::new(),
    false
  );

  let path1 = OsString::from(vec![ASSETS_BASE, "d"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "e"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let dir1 = EntryDir::from(entry1);
  let dir2 = EntryDir::from(entry2);
  match dir1.compare(&dir2) {
    Ok(()) => panic!("different directories compared, but reported as equal"),
    _  => assert!(true),
  }

  Ok(())
}

#[test]
pub fn test_same_directories() -> std::io::Result<()> {
  let options =  Options::new(
    9,
    false,
    false,
    OsString::new(),
    false
  );

  let path1 = OsString::from(vec![ASSETS_BASE, "a"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "c"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let dir1 = EntryDir::from(entry1);
  let dir2 = EntryDir::from(entry2);
  match dir1.compare(&dir2) {
    Ok(()) => assert!(true),
    _  => panic!("same directories compared, but reported as different"),
  }

  Ok(())
}
