use ::std::path;

#[derive(Debug, PartialEq, Eq)]
pub struct OpenFileOptions<'a> {
  pub start_path: Option<&'a path::Path>,
  pub title: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpenFileError {
  Unknown(Option<String>),
}
