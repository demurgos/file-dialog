extern crate libc;
extern crate gtk;
extern crate qt_core;
extern crate qt_widgets;

use qt_widgets::file_dialog::FileDialog;
use qt_widgets::cpp_utils::CppBox;
use qt_widgets::application::Application;
use qt_core::core_application::CoreApplication;

/// convert a Rust string to a QString
fn create_q_string(str: &str) -> qt_core::string::String {
  let mut bytearray = qt_core::byte_array::ByteArray::new(());
  for b in str.as_bytes() {
    bytearray.append(*b as libc::c_char);
  }
  qt_core::string::String::from_utf8(&bytearray)
}

fn main() {
  Application::create_and_exit(|app: &mut Application| {
    CoreApplication::set_attribute((qt_core::qt::ApplicationAttribute::UseHighDpiPixmaps, true));

    let parent: *mut qt_widgets::widget::Widget = std::ptr::null_mut();
    let title: qt_core::string::String = create_q_string("Open file");

    unsafe {
      let mut fd_box: CppBox<FileDialog> = qt_widgets::file_dialog::FileDialog::new_unsafe((parent, &title));
      fd_box.as_mut().set_accept_mode(qt_widgets::file_dialog::AcceptMode::Open);
      let result: libc::c_int = fd_box.as_mut().exec();
    }
    0
  })
}
