use cocoa_bindings;
use cocoa_bindings::base::{nil, NO, id as NsId};
use cocoa_bindings::foundation::{NSAutoreleasePool, NSString, NSInteger};
use cocoa_bindings::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSMenu, NSMenuItem,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps};
use libc;
use objc::runtime;
use std::ffi;
use std::option::Option;
use std::path;
use std::result::Result;
use types::{OpenFileError, OpenFileOptions};

#[allow(non_upper_case_globals)]
const NSModalResponseCancel: NSInteger = 0;
#[allow(non_upper_case_globals)]
const NSModalResponseOK: NSInteger = 1;

#[allow(non_upper_case_globals)]
pub fn open_file_sync(options: &OpenFileOptions) -> Result<Option<path::PathBuf>, OpenFileError> {
  unsafe {
    let pool = NSAutoreleasePool::new(nil);
    let app = NSApp();

    app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

    // create Menu Bar
    let menubar = NSMenu::new(nil).autorelease();
    let app_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(app_menu_item);
    app.setMainMenu_(menubar);

    // create Window
    let current_app = NSRunningApplication::currentApplication(nil);
    current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

    let dialog: *mut runtime::Object = msg_send![cocoa_bindings::base::class("NSOpenPanel"), openPanel];
    msg_send![dialog, setAllowsMultipleSelection:NO];
    let dir: *mut runtime::Object = match options.start_path {
      Some(start_path) => NSString::alloc(nil).init_str(&start_path.to_str().unwrap()),
      None => nil,
    };

    let modal_response: NSInteger = msg_send![dialog, runModalForDirectory:dir file:nil types:nil];
    let result = match modal_response {
      NSModalResponseCancel => Ok(None),
      NSModalResponseOK => {
        let url: *mut runtime::Object = msg_send![dialog, URL];
        let path_id: NsId = msg_send![url, path];
        let path_ptr: *const libc::c_char = path_id.UTF8String();
        match ffi::CStr::from_ptr(path_ptr).to_str() {
          Ok(path_str) => {
            Ok(Some(path::PathBuf::from(path_str)))
          },
          Err(e) => Err(OpenFileError::Unknown(Some(format!("Invalid path: {:?}", e)))),
        }
      },
      _ => Err(OpenFileError::Unknown(Some(format!("Invalid modal_response: {}", modal_response)))),
    };
    msg_send![pool, release];
    result
  }
}
