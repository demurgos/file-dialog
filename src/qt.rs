use cpp_utils::StaticCast;
use libc;
use qt_core;
use qt_widgets::file_dialog::{AcceptMode, FileDialog, FileMode};
use qt_widgets::cpp_utils::CppBox;
use qt_widgets::application::Application;
use qt_widgets::dialog::DialogCode;
use qt_core::core_application::{CoreApplication, CoreApplicationArgs};
use std::option::Option;
use std::path;
use std::result::Result;
use types::{OpenFileError, OpenFileOptions};

fn string_list_to_vector(string_list: qt_core::string_list::StringList) -> Result<Vec<String>, QtError> {
  let list: &qt_core::list::ListString = string_list.static_cast();
  let size: libc::c_int = list.size();
  if size < 0 {
    return Err(QtError::InvalidListSize(size))
  } else {
    Ok((0..size).map(|i| String::from(list.at(i))).collect())
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum QtError {
  InvalidDialogCode(i32),
  InvalidListSize(i32),
  InvalidFileCount(usize),
}

fn dialog_code_from_int(code: libc::c_int) -> Result<DialogCode, QtError> {
  match code {
    0 => Ok(DialogCode::Rejected),
    1 => Ok(DialogCode::Accepted),
    _ => Err(QtError::InvalidDialogCode(code)),
  }
}

fn with_application<T, F: FnOnce(&mut Application) -> T>(f: F) -> T {
  let mut args: CoreApplicationArgs = CoreApplicationArgs::empty();
  let mut app: CppBox<Application> = unsafe { Application::new(args.get()) };
  CoreApplication::set_attribute((qt_core::qt::ApplicationAttribute::UseHighDpiPixmaps, true));
  f(app.as_mut())
}

pub fn open_file_sync(options: &OpenFileOptions) -> Result<Option<path::PathBuf>, OpenFileError> {
  with_application(|_: &mut Application| {
    let mut fd_box: CppBox<FileDialog> = FileDialog::new();
    let fd: &mut FileDialog = fd_box.as_mut();

    fd.set_accept_mode(AcceptMode::Open);
    fd.set_file_mode(FileMode::ExistingFile);
    fd.set_window_modality(qt_core::qt::WindowModality::Application);
    if let Some(title) = options.title {
      let title: qt_core::string::String = qt_core::string::String::from(title);
      fd.set_window_title(&title);
    }

    match dialog_code_from_int(fd.exec()) {
      Ok(DialogCode::Rejected) => Ok(None),
      Ok(DialogCode::Accepted) => {
        match string_list_to_vector(fd.selected_files()) {
          Ok(files) => {
            match files.len() {
              1 => {
                let result: path::PathBuf = path::PathBuf::from(&files[0]);
                Ok(Some(result))
              },
              _ => Err(OpenFileError::Unknown(Some(format!("{:?}", QtError::InvalidFileCount(files.len()))))),
            }
          },
          Err(e) => Err(OpenFileError::Unknown(Some(format!("{:?}", e)))),
        }
      },
      Err(e) => Err(OpenFileError::Unknown(Some(format!("{:?}", e)))),
    }
  })
}
