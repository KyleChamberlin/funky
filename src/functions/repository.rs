use crate::functions::Slug;
use color_eyre::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub trait Repository {
  fn create(&self, id: &Slug, contents: &str) -> Result<()>;
  // fn read(&self, id: &Slug) -> Result<String>;
  // fn update(&self, id: &Slug, contents: &str) -> Result<()>;
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
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use std::cell::RefCell;
  use std::collections::HashMap;
  use std::str::FromStr;
  use tempfile::tempdir;

  pub struct MockRepository {
    pub functions: RefCell<HashMap<String, String>>,
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
}
