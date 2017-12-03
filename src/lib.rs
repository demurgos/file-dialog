#[cfg(target_os = "linux")]
extern crate cpp_utils;
#[cfg(target_os = "linux")]
extern crate gtk as gtk_bindings;
#[cfg(target_os = "linux")]
extern crate glib;
#[cfg(target_os = "linux")]
extern crate libc;
#[cfg(target_os = "linux")]
extern crate qt_core;
#[cfg(target_os = "linux")]
extern crate qt_widgets;

extern crate detect_desktop_environment;

use ::std::path;

mod types;

pub use types::OpenFileError;
pub use types::OpenFileOptions;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub mod qt;
#[cfg(target_os = "linux")]
pub mod gtk;

#[cfg(target_os = "linux")]
pub fn open_file_sync(options: &OpenFileOptions) -> Result<Option<path::PathBuf>, OpenFileError> {
  gtk::open_file_sync(options)
}
