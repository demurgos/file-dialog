use gtk_bindings;
use gtk_bindings::{Window, DialogExt, FileChooserAction, FileChooserDialog, FileChooserExt, ResponseType, WidgetExt};
use glib::translate::FromGlib;
use std::option::Option;
use std::path;
use std::result::Result;
use types::{OpenFileError, OpenFileOptions};

/// @see https://github.com/GNOME/gtk/blob/bcc77e169cb0a5219ef5c1f0554b90978ca0d17f/gtk/gtkfilechooserbutton.c#L106
pub const DEFAULT_TITLE: &str = "Select a File";

pub fn open_file_sync(options: &OpenFileOptions) -> Result<Option<path::PathBuf>, OpenFileError> {
  if let Err(err) = gtk_bindings::init() {
    return Err(OpenFileError::Unknown(Some(format!("{:?}", err))));
  }

  let title = options.title.or(Some(DEFAULT_TITLE));

  let dialog = FileChooserDialog::with_buttons::<Window>(
    title,
    Option::None,
    FileChooserAction::Open,
    &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)]
  );

  let result = match ResponseType::from_glib(dialog.run()) {
    ResponseType::DeleteEvent => Ok(None), // Close the window
    ResponseType::Cancel => Ok(None), // Cancel button
    ResponseType::Accept => Ok(dialog.get_filename()),
    response_type => Err(OpenFileError::Unknown(Some(format!("Unexpected response type: {:?}", response_type))))
  };

  dialog.destroy();
  result
}
