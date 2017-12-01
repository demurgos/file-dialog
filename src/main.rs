#[macro_use]
extern crate objc;

extern crate cocoa;

use cocoa::base::{selector, nil, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString, NSInteger};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSWindowStyleMask,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps};

fn main() {
  unsafe {
    let _pool = NSAutoreleasePool::new(nil);
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

    let dialog: *mut objc::runtime::Object = msg_send![cocoa::base::class("NSOpenPanel"), openPanel];
    msg_send![dialog, setAllowsMultipleSelection:objc::runtime::YES];
    let allow_mult: objc::runtime::BOOL = msg_send![dialog, allowsMultipleSelection];
    let dir = NSString::alloc(nil).init_str("/Users/demurgos");
    let selected_count: NSInteger = msg_send![dialog, runModalForDirectory:dir file:nil types:nil];
    println!("{:?}", selected_count);
  }
}
