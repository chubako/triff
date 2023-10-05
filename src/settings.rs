//const CHUNK_SIZE : usize = 1;
//const CHUNK_SIZE : usize = 0x4000;

#[derive(Debug)]
pub enum SuccessKind {
  NonRegular,
  SameInums,
  SameContent,
  SkipContent,
  Other
}

//pub const CHUNK_SIZE : usize = 8192;
pub const CHUNK_LENGTH : usize = 2;

pub const META_BLOCK_SIZE : u64 = 512;

pub const ERROR_NOT_FOUND          : &str = "xF";
pub const ERROR_CANT_READ_METADATA : &str = "xM";
pub const ERROR_UNREADABLE         : &str = "xR";
pub const ERROR_UNACCESSIBLE       : &str = "xA";
pub const ERROR_FILE_TYPE          : &str = "xT";
pub const ERROR_SYMLINKS_DIFFERENT : &str = "xL";
pub const ERROR_SYMLINK_READ       : &str = "xl";
pub const ERROR_CONTENT            : &str = "xC";
//pub const ERROR_UNKNOWN            : &str = "xK";
pub const ERROR_ONLY_PATH          : &str = "x~";
pub const ERROR_DIRECTORY          : &str = "X ";

pub const SUCCESS_ONLY_PATH   : &str = "✓~";
pub const SUCCESS_SKIP_CONTENT: &str = "✓.";
pub const SUCCESS_INODE       : &str = "✓≡";
pub const SUCCESS_SYMLINK     : &str = "✓→";
pub const SUCCESS_CONTENT     : &str = "✓@";
pub const SUCCESS_DIR         : &str = "✓+";

pub const WARN_PERMISSION   : &str = "!P";
pub const WARN_OWNERS       : &str = "!O";
pub const WARN_GROUPS       : &str = "!G";
pub const WARN_NOT_IN       : &str = "!N";
pub const WARN_SIZE         : &str = "!S";
pub const WARN_REAL_SIZE    : &str = "!Z";
pub const WARN_REGULAR_FILE : &str = "!R";

pub const NOOP : &str = ": ";

//pub const EXIT_CODE_FAILED      : i32 = 20;
//pub const EXIT_CODE_INTERRUPTED : i32 = 21;
