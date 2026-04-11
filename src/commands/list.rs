use crate::functions::repository::{FileSystemRepository, Repository};
use color_eyre::eyre::Result;
use std::path::Path;

pub fn list(funky_dir: &Path) -> Result<()> {
  let repo = FileSystemRepository::new(funky_dir);
  let functions = repo.list()?;
  for name in functions {
    println!("{}", name);
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use tempfile::tempdir;

  #[test]
  fn test_list_empty_dir() {
    let tmp_dir = tempdir().unwrap();
    let repo = FileSystemRepository::new(tmp_dir.path());
    let result = repo.list().unwrap();
    assert!(result.is_empty());
  }

  #[test]
  fn test_list_with_functions() {
    let tmp_dir = tempdir().unwrap();
    fs::write(tmp_dir.path().join("my-func.zsh"), "echo hello").unwrap();
    fs::write(tmp_dir.path().join("other-func.zsh"), "echo world").unwrap();
    let repo = FileSystemRepository::new(tmp_dir.path());
    let mut result = repo.list().unwrap();
    result.sort();
    assert_eq!(result, vec!["my-func", "other-func"]);
  }

  #[test]
  fn test_list_ignores_non_zsh_files() {
    let tmp_dir = tempdir().unwrap();
    fs::write(tmp_dir.path().join("my-func.zsh"), "echo hello").unwrap();
    fs::write(tmp_dir.path().join("readme.txt"), "not a function").unwrap();
    let repo = FileSystemRepository::new(tmp_dir.path());
    let result = repo.list().unwrap();
    assert_eq!(result, vec!["my-func"]);
  }
}
