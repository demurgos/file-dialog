extern crate nd_open_file;

use nd_open_file::{open_file_sync, OpenFileOptions};

fn main() {
  let result = open_file_sync(&OpenFileOptions {
    start_path: Option::None,
    title: Option::Some("Test it"),
  });
  println!("{:?}", result);
}
