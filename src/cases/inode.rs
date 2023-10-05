use std::ffi::OsString;
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::fs::MetadataExt;

pub fn check_inode(path1: &OsString, path2: &OsString) -> Result<bool, Error> {
    let meta1 = fs::symlink_metadata(path1)?;
    let inode1 = meta1.ino();
    let device1= meta1.dev();

    let meta2 = std::fs::symlink_metadata(path2)?;
    let inode2 = meta2.ino();
    let device2= meta2.dev();

    if device1==device2 && inode1==inode2 {
      Ok(true)
    } else {
      Err(Error::new(ErrorKind::Other, ""))
    }
}
