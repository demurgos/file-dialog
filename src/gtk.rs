//extern crate libc;
//extern crate gtk;
//extern crate qt_core;
//extern crate qt_widgets;
//
//use qt_widgets::file_dialog::FileDialog;
//use qt_widgets::cpp_utils::CppBox;
//use qt_core::core_application::CoreApplication;
//use qt_core::variant::Variant;
//use qt_core::variant_animation::VariantAnimation;
//use qt_core::connection::Signal;
//use qt_core::slots::SlotVariantRef;
//use gtk::DialogExt;
//use std::ops::Deref;
//
///// convert a Rust string to a QString
//fn create_q_string(str: &str) -> qt_core::string::String {
//  let mut bytearray = qt_core::byte_array::ByteArray::new(());
//  for b in str.as_bytes() {
//    bytearray.append(*b as libc::c_char);
//  }
//  qt_core::string::String::from_utf8(&bytearray)
//}
//
//fn main() {
//  CoreApplication::create_and_exit(|app: &mut CoreApplication| {
//    println!("Before");
//    CoreApplication::set_attribute((qt_core::qt::ApplicationAttribute::UseHighDpiPixmaps, true));
//
//    unsafe {
//      let parent: *mut qt_widgets::widget::Widget = std::ptr::null_mut();
//      println!("With parent");
//      let mut fd_box: CppBox<FileDialog> = qt_widgets::file_dialog::FileDialog::new_unsafe((parent));
//    }
//    println!("After");
//    0
//  })
////  let fd: &mut FileDialog = fd_box.as_mut();
////  println!("{:?}", "hoy");
////  fd.set_accept_mode(qt_widgets::file_dialog::AcceptMode::Open);
////  println!("{:?}", "hoy");
//
////  CoreApplication::create_and_exit(|app| {
////
////
////    unsafe {
////
////      let caption: qt_core::string::String = create_q_string("Open File");
////      qt_widgets::file_dialog::FileDialog::get_open_file_name_unsafe((parent, &caption));
////    }
////    0
////  })
//}
//
////fn gtk_f() {
//////  let window = ApplicationWindow::new(application);
////  let init_result = gtk::init();
////  if init_result.is_err() {
////    return;
////  }
////  let dialog = gtk::FileChooserDialog::new::<gtk::ApplicationWindow>(Option::Some("Open File"), Option::None, gtk::FileChooserAction::Open);
////  let result = dialog.run();
////}
