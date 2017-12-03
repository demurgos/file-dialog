use ole32;
use std::io;
use std::option::Option;
use std::path;
use std::ptr;
use std::result::Result;
use types::{OpenFileError, OpenFileOptions};
use uuid;
use widestring::WideCString;
use winapi;
use winapi::winerror;

#[inline]
fn check(result: winapi::HRESULT) -> Result<(), io::Error> {
  if result < 0 {
    Err(io::Error::from_raw_os_error(result))
  } else {
    Ok(())
  }
}

fn with_com<T, F: FnOnce() -> Result<T, OpenFileError>>(f: F) -> Result<T, OpenFileError> {
  if let Err(e) = check(unsafe {
    ole32::CoInitializeEx(
      ptr::null_mut(),
      winapi::COINIT_APARTMENTTHREADED | winapi::COINIT_DISABLE_OLE1DDE
    )
  }) {
    return Err(OpenFileError::Unknown(Some(format!("Unable to initialize COM: {:?}", e))));
  }
  let result = f();
  unsafe { ole32::CoUninitialize() }
  result
}

fn with_file_dialog<T, F: FnOnce(&mut winapi::IFileOpenDialog) -> Result<T, OpenFileError>>(f: F) -> Result<T, OpenFileError> {
  let mut file_dialog: *mut winapi::IFileOpenDialog = ptr::null_mut();
  if let Err(e) = check(unsafe {
    ole32::CoCreateInstance(
      &uuid::CLSID_FileOpenDialog,
      ptr::null_mut(),
      winapi::CLSCTX_ALL,
      &uuid::IID_IFileOpenDialog,
      &mut file_dialog as *mut _ as *mut _
    )
  }) {
    return Err(OpenFileError::Unknown(Some(format!("Unable to instantiate IFileOpenDialog: {:?}", e))));
  }
  let result = f(unsafe { file_dialog.as_mut() }.unwrap());
  unsafe {
    // `Release` returns the new reference count, it can be ignored
    (*file_dialog).Release();
  }
  result
}

// struct ComInitialized(*mut ());
// impl Drop for ComInitialized {
//   fn drop(&mut self) {
//     unsafe { ole32::CoUninitialize() };
//   }
// }


// TODO: Update to winapi 0.3 and import it from there
// See: https://github.com/retep998/winapi-rs/blob/7b791b37e3a81eff335fdd6eb720a08dbd82f623/src/shared/winerror.rs#L2954
#[allow(non_snake_case)]
#[inline]
fn HRESULT_FROM_WIN32(x: winapi::DWORD) -> winapi::HRESULT {
  let hr = x as winapi::HRESULT;
  if hr <= 0 {
    hr
  } else {
    ((x & 0x0000FFFF) | ((winapi::FACILITY_WIN32 as u32) << 16) | 0x80000000) as winapi::HRESULT
  }
}

// https://msdn.microsoft.com/en-us/library/windows/desktop/bb776913(v=vs.85).aspx#usage
pub fn open_file_sync(_options: &OpenFileOptions) -> Result<Option<path::PathBuf>, OpenFileError> {
  with_com(|| {
    with_file_dialog(|file_dialog: &mut winapi::IFileOpenDialog| {
      // https://msdn.microsoft.com/en-us/library/windows/desktop/bb761688(v=vs.85).aspx
      if let Err(e) = check(unsafe { file_dialog.Show(ptr::null_mut()) }) {
        #[allow(non_snake_case)]
        let HRESULT_CANCELLED: winapi::HRESULT = HRESULT_FROM_WIN32(winerror::ERROR_CANCELLED);
        return match e.raw_os_error() {
          Some(os_error) if os_error == HRESULT_CANCELLED => Ok(None),
          _ => Err(OpenFileError::Unknown(Some(format!("Unexpected Show result: {:?}", e))))
        };
      }
      let mut shell_item: *mut winapi::IShellItem = ptr::null_mut();
      match check(unsafe { file_dialog.GetResult(&mut shell_item as *mut _ as *mut _) }) {
        Err(e) => Err(OpenFileError::Unknown(Some(format!("Unable to read shell item: {:?}", e)))),
        Ok(_) => {
//          let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
          let mut file_path: winapi::LPWSTR = ptr::null_mut();
          let result = match check(unsafe { (*shell_item).GetDisplayName(winapi::SIGDN_FILESYSPATH, &mut file_path as *mut _ as *mut _) }) {
            Err(e) => Err(OpenFileError::Unknown(Some(format!("Unable to read file path: {:?}", e)))),
            Ok(_) => {
              let wide: WideCString = unsafe { WideCString::from_ptr_str(file_path) };
              let path: path::PathBuf = path::PathBuf::from(wide.to_os_string());
              unsafe { ole32::CoTaskMemFree(file_path as *mut _); }
              Ok(Some(path))
            }
          };
          unsafe { (*shell_item).Release(); }
          result
        }
      }
    })
  })
}
