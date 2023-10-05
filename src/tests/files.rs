use std::ffi::OsString;

use super::settings::SuccessKind;
use super::Options;
use super::entries::{Entry, EntryFile};

const ASSETS_BASE:&str = "src/tests/assets/test_01_just_files";

#[test]
pub fn test_files_same_path() -> std::io::Result<()> {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file1"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "a/file1"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_file1 = EntryFile::from(entry1);
  let entry_file2 = EntryFile::from(entry2);
  match entry_file1.compare(&entry_file2) {
    Ok(SuccessKind::SameInums) => assert!(true),
    _ => panic!("Files have same inum, but comparison didn't notice that"),
  }

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file1"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/file1"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_file1 = EntryFile::from(entry1);
  let entry_file2 = EntryFile::from(entry2);
  match entry_file1.compare(&entry_file2) {
    Ok(SuccessKind::SameInums) => assert!(true),
    _ => panic!("Files have same inum, but comparison didn't notice that"),
  }

  Ok(())
}

#[test]
pub fn test_files_same_inum() -> std::io::Result<()> {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file1"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/file1"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_file1 = EntryFile::from(entry1);
  let entry_file2 = EntryFile::from(entry2);
  match entry_file1.compare(&entry_file2) {
    Ok(SuccessKind::SameInums) => assert!(true),
    _ => panic!("Files have same inum, but comparison didn't notice that"),
  }

  Ok(())
}

#[test]
pub fn test_files_sizes() -> std::io::Result<()> {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file2"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/file2"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_file1 = EntryFile::from(entry1);
  let entry_file2 = EntryFile::from(entry2);
  match entry_file1.compare_sizes(&entry_file2) {
    true  => panic!("Files have differente sizes, but reported them as equal"),
    false => assert!(true),
  }

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file3"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/file3"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_file1 = EntryFile::from(entry1);
  let entry_file2 = EntryFile::from(entry2);
  match entry_file1.compare_sizes(&entry_file2) {
    true  => assert!(true),
    false => panic!("Files have same size, but reported them as different"),
  }

  Ok(())
}

#[test]
pub fn test_files_real_sizes() -> std::io::Result<()> {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file4"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/file4"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_file1 = EntryFile::from(entry1);
  let entry_file2 = EntryFile::from(entry2);
  match entry_file1.compare_sizes(&entry_file2) {
    true  => assert!(true),
    false => panic!("Files have same size, but reported them as different"),
  }

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file5"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/file5"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_file1 = EntryFile::from(entry1);
  let entry_file2 = EntryFile::from(entry2);
  match entry_file1.compare_sizes(&entry_file2) {
    true  => panic!("Files have differente sizes, but reported them as equal"),
    false => assert!(true),
  }

  Ok(())
}

#[test]
pub fn test_files_contents() -> std::io::Result<()> {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file6"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/file6"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_file1 = EntryFile::from(entry1);
  let entry_file2 = EntryFile::from(entry2);
  match entry_file1.compare_contents(&entry_file2) {
    Ok(SuccessKind::SameContent) => assert!(true),
    _ => panic!("Files have same content, but comparison didn't notice that"),
  }

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file7"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/file7"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_file1 = EntryFile::from(entry1);
  let entry_file2 = EntryFile::from(entry2);
  match entry_file1.compare_contents(&entry_file2) {
    Ok(_) => panic!("Files have same different content, but comparison didn't notice that"),
    _ => assert!(true),
  }

  Ok(())
}
