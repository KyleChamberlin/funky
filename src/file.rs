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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_get_dir_creates_dir() {
        let tmp_dir = tempdir().unwrap();
        let dir_path = tmp_dir.path().join("new_dir");
        assert!(!dir_path.exists());
        let result = get_dir(dir_path.to_str().unwrap().to_string()).unwrap();
        assert!(result.exists());
        assert!(result.is_dir());
    }

    #[test]
    fn test_get_dir_exists() {
        let tmp_dir = tempdir().unwrap();
        let dir_path = tmp_dir.path();
        assert!(dir_path.exists());
        let result = get_dir(dir_path.to_str().unwrap().to_string()).unwrap();
        assert_eq!(result, dir_path);
    }

    #[test]
    fn test_get_dir_is_file() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("a_file");
        File::create(&file_path).unwrap();
        let result = get_dir(file_path.to_str().unwrap().to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_file_exists() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("a_file");
        File::create(&file_path).unwrap();
        let result = get_file(file_path.to_str().unwrap().to_string()).unwrap();
        assert_eq!(result, file_path);
    }

    #[test]
    fn test_get_file_not_exists() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("a_file");
        let result = get_file(file_path.to_str().unwrap().to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_file_is_dir() {
        let tmp_dir = tempdir().unwrap();
        let dir_path = tmp_dir.path();
        let result = get_file(dir_path.to_str().unwrap().to_string());
        assert!(result.is_err());
    }
}
