//#![allow(unused)]
use std::io;
use std::fs;
use std::process;

use clap::Parser;
use std::ffi::OsString;

use triff::Options;
use triff::cases::diff;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t=3)]
    verbosity: u8,

    #[clap(short, long)]
    nocheckcontent: bool,

    #[clap(short, long)]
    onlypath: bool,

    #[clap(long)]
    regular_info: bool,

    #[clap(long, default_value="")]
    logfile: OsString,

    path1: OsString,
    path2: OsString,
}

fn exit(exit_code: i32) {
  process::exit(exit_code)
}

fn shutdown() {
  exit(20)
}

fn result_ok() {
  println!("success");
  exit(0)
}

fn result_mismatch() {
  println!("mismatch");
  exit(1)
}

fn result_content_mismatch() {
    println!("content_mismatch");
    exit(2)
}

fn result_not_found() {
    println!("target not found");
    exit(3)
}

fn result_same_abs_path() {
    println!("both paths point to the same absolute paths");
    exit(4)
}

fn result_unhandled() {
    println!("other");
    exit(10)
}

async fn run(path1: OsString, path2: OsString, options: Options) -> std::io::Result<()> {
  tokio::spawn(async move {
    tokio::signal::ctrl_c().await.unwrap();
    shutdown();
  });

  let abs_path1 = fs::canonicalize(&path1)?;
  let abs_path2 = fs::canonicalize(&path2)?;
  if abs_path1==abs_path2 {
    result_same_abs_path()
  }

  diff(&path1, &path2, &options)
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let path1 = args.path1;
  let path2 = args.path2;

  let options =  Options::new(
    args.verbosity,
    args.nocheckcontent,
    args.onlypath,
    args.logfile,
    args.regular_info,
  );

  match run(path1, path2, options).await {
      Ok(_) => result_ok(),
      Err(code) => match code.kind() {
          io::ErrorKind::InvalidData=> result_mismatch(),
          io::ErrorKind::Other => result_content_mismatch(),
          io::ErrorKind::NotFound => result_not_found(),
          _ => result_unhandled()
      }
  }
}
