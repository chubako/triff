use std::ffi::OsString;

use crate::Options;
use crate::entries::{Entry, EntrySymlink, EntryDir, EntryFile, EntryNonregular};

pub fn compare_entries(entry1: Entry, entry2: Entry) -> std::io::Result<()> {
    entry1.compare(&entry2)?;

    if entry1.meta.is_symlink() {
      let symlink1 = EntrySymlink::from(entry1);
      let symlink2 = EntrySymlink::from(entry2);
      symlink1.compare(&symlink2)?;
    } else if entry1.meta.is_dir() {
      let dir1 = EntryDir::from(entry1);
      let dir2 = EntryDir::from(entry2);
      dir1.compare(&dir2)?;
    } else if entry1.meta.is_file() {
      let file1 = EntryFile::from(entry1);
      let file2 = EntryFile::from(entry2);
      file1.compare(&file2)?;
    } else {
      let nonregular1 = EntryNonregular::from(entry1);
      let nonregular2 = EntryNonregular::from(entry2);
      nonregular1.compare(&nonregular2)?;
    }

    return Ok(())
}

pub fn diff(target1: &OsString, target2: &OsString, options: &Options) -> std::io::Result<()> {
    let entry1 = Entry::new(target1, options)?;
    let entry2 = Entry::new(target2, options)?;

    compare_entries(entry1, entry2)
}
