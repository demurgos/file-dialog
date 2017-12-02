#[cfg(target_os = "linux")]
extern crate cpp_utils;
#[cfg(target_os = "linux")]
extern crate libc;
#[cfg(target_os = "linux")]
extern crate qt_core;
#[cfg(target_os = "linux")]
extern crate qt_widgets;

use ::std::path;

mod types;

pub use types::OpenFileError;
pub use types::OpenFileOptions;

#[cfg(target_os = "linux")]
mod qt;

#[cfg(target_os = "linux")]
pub fn open_file_sync(options: &OpenFileOptions) -> Result<Option<path::PathBuf>, OpenFileError> {
  qt::open_file_sync(options)
}
