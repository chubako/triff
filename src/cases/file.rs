use std::ffi::OsString;
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::fs::MetadataExt;

use std::io::{BufReader, Read};

use crate::settings;
use settings::{CHUNK_LENGTH};

use std::fmt::Write;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};

pub fn check_file_types(mode1: u32, mode2: u32) -> bool {
    let file_type1 = mode1 & libc::S_IFMT;
    let file_type2 = mode2 & libc::S_IFMT;
    file_type1==file_type2
}

pub fn check_content(path1: &OsString, path2: &OsString) -> Result<(), Error> {
    let file1 = std::fs::File::open(path1)?;

    let meta1 = fs::symlink_metadata(path1).expect("Failed to get file metadata");
    let size1 = meta1.size();
    let blksize : usize = meta1.blksize() as usize;

    let chunk_len : usize = CHUNK_LENGTH * blksize;

    let mut chunk1 : Vec<u8> = Vec::new();
    chunk1.resize(chunk_len, 0u8);

    let mut reader1 = BufReader::new(file1);

    let file2 = std::fs::File::open(path2)?;
    let mut chunk2 : Vec<u8> = Vec::new();
    chunk2.resize(chunk_len, 0u8);

    let mut reader2 = BufReader::new(file2);

    let m = MultiProgress::new();
    let pb1 = m.add(ProgressBar::new(size1));
    let pb2 = m.insert_after(&pb1, ProgressBar::new(size1));

    pb1.set_style(ProgressStyle::with_template("{msg:.magenta}")
      .unwrap());
    pb1.set_message(format!("\n{}", path1.as_os_str().to_string_lossy()));

    pb2.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise} |{eta_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} | {bytes_per_sec}")
      .unwrap()
      .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
      .progress_chars("#>-"));

    loop {
      let n = reader1.read(&mut chunk1)?;
      let m = reader2.read(&mut chunk2)?;

      if n != m {
        return Err(Error::new(ErrorKind::Other, ""));
      }

      if n == 0 { break; }
      if m == 0 { break; }

      if chunk1 != chunk2 {
        return Err(Error::new(ErrorKind::Other, ""));
      }

      pb2.inc(n as u64);
    }
    m.clear().unwrap();
    Ok(())
}
