use std::ffi::OsString;
use std::io::ErrorKind;

use super::Options;
use super::entries::{Entry, EntrySymlink};

const ASSETS_BASE:&str = "src/tests/assets/test_03_and_symlinks";

#[test]
pub fn test1_symlink_working_existence() {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path_doesnt_exist = OsString::from(vec![ASSETS_BASE, "a/prueba0"].join("/"));

  match Entry::new(&path_doesnt_exist, &options) {
    Ok(_) => panic!("Symlink path doesn't exist, but EntrySymlink created"),
    Err(error) => match error.kind() {
      ErrorKind::NotFound=> assert!(true),
      _ => panic!("Symlink could not be created by unknow reason")
    }
  };

  let path_exists = OsString::from(vec![ASSETS_BASE, "a/prueba1"].join("/"));

  match Entry::new(&path_exists, &options) {
    Ok(_) => assert!(true),
    Err(_) => panic!("Symlink path exists, but EntrySymlink could not be created"),
  };
}

#[test]
pub fn test2_symlink_working_same() -> std::io::Result<()> {
  let path1 = OsString::from(vec![ASSETS_BASE, "a/prueba1"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/prueba1"].join("/"));

  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;

  let symlink1 = EntrySymlink::from(entry1);
  let symlink2 = EntrySymlink::from(entry2);

  symlink1.compare(&symlink2)
}

#[test]
pub fn test3_symlink_working_different() -> std::io::Result<()> {
  let path1 = OsString::from(vec![ASSETS_BASE, "a/prueba1"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "c/prueba1"].join("/"));

  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;

  let symlink1 = EntrySymlink::from(entry1);
  let symlink2 = EntrySymlink::from(entry2);

  match symlink1.compare(&symlink2) {
    Ok(_) => panic!("Symlinks different, but comparison says they are the same"),
    Err(_) => assert!(true),
  };
  Ok(())
}

#[test]
pub fn test4_symlink_broken_existence() {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path_exists = OsString::from(vec![ASSETS_BASE, "a/prueba2"].join("/"));

  match Entry::new(&path_exists, &options) {
    Ok(_) => assert!(true),
    Err(_) => panic!("Symlink path doesn't exist, but EntrySymlink created"),
  };
}

#[test]
pub fn test5_symlink_broken_same() -> std::io::Result<()> {
  let path1 = OsString::from(vec![ASSETS_BASE, "a/prueba2"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/prueba2"].join("/"));

  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;

  let symlink1 = EntrySymlink::from(entry1);
  let symlink2 = EntrySymlink::from(entry2);

  symlink1.compare(&symlink2)
}

#[test]
pub fn test6_symlink_broken_different() -> std::io::Result<()> {
  let path1 = OsString::from(vec![ASSETS_BASE, "a/prueba2"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "c/prueba2"].join("/"));

  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;

  let symlink1 = EntrySymlink::from(entry1);
  let symlink2 = EntrySymlink::from(entry2);

  match symlink1.compare(&symlink2) {
    Ok(_) => panic!("Symlinks different, but comparison says they are the same"),
    Err(_) => assert!(true),
  };
  Ok(())
}

#[test]
pub fn test7_symlink_as_entry_working_existence() {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path_exists = OsString::from(vec![ASSETS_BASE, "a/prueba1"].join("/"));

  match Entry::new(&path_exists, &options) {
    Ok(_) => assert!(true),
    Err(_) => panic!("Symlink path exists, but Entry could not be created"),
  };
}

#[test]
pub fn test8_symlink_as_entry_broken_existence() {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path_exists = OsString::from(vec![ASSETS_BASE, "a/prueba2"].join("/"));

  match Entry::new(&path_exists, &options) {
    Ok(_) => assert!(true),
    Err(_) => panic!("Symlink path exists, but Entry could not be created"),
  };
}
