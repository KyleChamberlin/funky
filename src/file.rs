use color_eyre::{eyre::eyre, Result};
use std::{fs, path::PathBuf};

pub fn get_dir(path: String) -> Result<PathBuf> {
  let expanded_path = shellexpand::full(&path)?;
  let dir = PathBuf::from(expanded_path.to_string());

  if dir.exists() {
    if !dir.is_dir() {
      return Err(eyre!("provided directory {path} is not a directory"));
    }
  } else {
    fs::create_dir_all(&dir)?;
  };

  Ok(dir)
}

pub fn get_file(path: String) -> Result<PathBuf> {
  let expanded_path = shellexpand::full(&path)?;
  let file = PathBuf::from(expanded_path.to_string());

  if file.exists() {
    if file.is_dir() {
      return Err(eyre!("provided file {path} is a directory"));
    }
  } else {
    return Err(eyre!("file {path} does not exist"));
  };

  Ok(file)
}
