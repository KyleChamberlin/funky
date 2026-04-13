use crate::functions::Slug;
use color_eyre::Result;
use color_eyre::eyre::eyre;
use std::fs;
use std::path::{Path, PathBuf};

pub trait Repository {
  fn create(&self, id: &Slug, contents: &str) -> Result<()>;
  fn list(&self) -> Result<Vec<String>>;
  fn read(&self, id: &Slug) -> Result<String>;
  fn update(&self, id: &Slug, contents: &str) -> Result<()>;
  // fn delete(&self, id: &Slug) -> Result<()>;
}

pub struct FileSystemRepository {
  base_path: PathBuf,
}

impl FileSystemRepository {
  pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
    Self {
      base_path: base_path.as_ref().to_path_buf(),
    }
  }
}

impl Repository for FileSystemRepository {
  fn create(&self, id: &Slug, contents: &str) -> Result<()> {
    let file_path = self.base_path.join(format!("{}.zsh", id));
    fs::write(file_path, contents)?;
    Ok(())
  }

  fn list(&self) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(&self.base_path)? {
      let entry = entry?;
      let path = entry.path();
      if path.extension().and_then(|e| e.to_str()) == Some("zsh")
        && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
      {
        names.push(stem.to_string());
      }
    }
    Ok(names)
  }

  fn read(&self, id: &Slug) -> Result<String> {
    let file_path = self.base_path.join(format!("{}.zsh", id));
    fs::read_to_string(&file_path).map_err(Into::into)
  }

  fn update(&self, id: &Slug, contents: &str) -> Result<()> {
    let file_path = self.base_path.join(format!("{}.zsh", id));
    if !file_path.exists() {
      return Err(eyre!("Function '{}' not found", id));
    }
    fs::write(file_path, contents)?;
    Ok(())
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use color_eyre::eyre::eyre;
  use std::cell::RefCell;
  use std::collections::HashMap;
  use std::str::FromStr;
  use tempfile::tempdir;

  pub struct MockRepository {
    pub functions: RefCell<HashMap<String, String>>,
  }

  impl Default for MockRepository {
    fn default() -> Self {
      Self::new()
    }
  }

  impl MockRepository {
    pub fn new() -> Self {
      Self {
        functions: RefCell::new(HashMap::new()),
      }
    }
  }

  impl Repository for MockRepository {
    fn create(&self, id: &Slug, contents: &str) -> Result<()> {
      self
        .functions
        .borrow_mut()
        .insert(id.to_string(), contents.to_string());
      Ok(())
    }

    fn list(&self) -> Result<Vec<String>> {
      Ok(self.functions.borrow().keys().cloned().collect())
    }

    fn read(&self, id: &Slug) -> Result<String> {
      self
        .functions
        .borrow()
        .get(&id.to_string())
        .cloned()
        .ok_or_else(|| eyre!("Function '{}' not found", id))
    }

    fn update(&self, id: &Slug, contents: &str) -> Result<()> {
      let mut functions = self.functions.borrow_mut();
      if !functions.contains_key(&id.to_string()) {
        return Err(eyre!("Function '{}' not found", id));
      }
      functions.insert(id.to_string(), contents.to_string());
      Ok(())
    }
  }

  #[test]
  fn test_fs_repo_create() {
    let tmp_dir = tempdir().unwrap();
    let repo = FileSystemRepository::new(tmp_dir.path());
    let slug = Slug::from_str("test-func").unwrap();
    let contents = "echo 'hello'";

    repo.create(&slug, contents).unwrap();

    let file_path = tmp_dir.path().join("test-func.zsh");
    assert!(file_path.exists());
    let read_contents = fs::read_to_string(file_path).unwrap();
    assert_eq!(read_contents, contents);
  }

  #[test]
  fn test_fs_repo_list() {
    let tmp_dir = tempdir().unwrap();
    let repo = FileSystemRepository::new(tmp_dir.path());

    let result = repo.list().unwrap();
    assert!(result.is_empty());

    fs::write(tmp_dir.path().join("alpha.zsh"), "echo alpha").unwrap();
    fs::write(tmp_dir.path().join("beta.zsh"), "echo beta").unwrap();

    let mut result = repo.list().unwrap();
    result.sort();
    assert_eq!(result, vec!["alpha", "beta"]);
  }

  #[test]
  fn test_fs_repo_read() {
    let tmp_dir = tempdir().unwrap();
    let repo = FileSystemRepository::new(tmp_dir.path());
    let slug = Slug::from_str("test-func").unwrap();
    fs::write(tmp_dir.path().join("test-func.zsh"), "echo hello").unwrap();

    let contents = repo.read(&slug).unwrap();
    assert_eq!(contents, "echo hello");
  }

  #[test]
  fn test_fs_repo_read_not_found() {
    let tmp_dir = tempdir().unwrap();
    let repo = FileSystemRepository::new(tmp_dir.path());
    let slug = Slug::from_str("nonexistent").unwrap();

    assert!(repo.read(&slug).is_err());
  }

  #[test]
  fn test_fs_repo_update() {
    let tmp_dir = tempdir().unwrap();
    let slug = Slug::from_str("test-func").unwrap();
    fs::write(tmp_dir.path().join("test-func.zsh"), "echo old").unwrap();

    let repo = FileSystemRepository::new(tmp_dir.path());
    repo.update(&slug, "echo new").unwrap();

    let contents = fs::read_to_string(tmp_dir.path().join("test-func.zsh")).unwrap();
    assert_eq!(contents, "echo new");
  }

  #[test]
  fn test_fs_repo_update_not_found() {
    let tmp_dir = tempdir().unwrap();
    let slug = Slug::from_str("nonexistent").unwrap();
    let repo = FileSystemRepository::new(tmp_dir.path());

    assert!(repo.update(&slug, "content").is_err());
  }
}
