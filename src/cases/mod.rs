pub mod permissions;
pub mod inode;
pub mod file;
pub mod compare;

pub use crate::cases::permissions::check_permissions;
pub use crate::cases::inode::check_inode;
pub use crate::cases::file::check_file_types;
pub use crate::cases::file::check_content;
pub use crate::cases::compare::diff;
