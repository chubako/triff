use std::ffi::OsString;

use super::settings::SuccessKind;
use super::Options;
use super::entries::{Entry, EntryNonregular};

const ASSETS_BASE:&str = "src/tests/assets/test_02_nonregular";

#[test]
pub fn test_special_file() -> std::io::Result<()> {
  let options =  Options::new(
    0,
    false,
    false,
    OsString::new(),
    false
  );

  let path1 = OsString::from(vec![ASSETS_BASE, "a/file8_char"].join("/"));
  let path2 = OsString::from(vec![ASSETS_BASE, "b/file8_char"].join("/"));
  let entry1 = Entry::new(&path1, &options)?;
  let entry2 = Entry::new(&path2, &options)?;
  let entry_nonregular1 = EntryNonregular::from(entry1);
  let entry_nonregular2 = EntryNonregular::from(entry2);
  match entry_nonregular1.compare(&entry_nonregular2) {
    Ok(SuccessKind::NonRegular) => assert!(true),
    _ => panic!("non-regular file compared, but didn't succeed")
  }

  Ok(())
}
