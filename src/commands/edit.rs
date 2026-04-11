use crate::functions::repository::{FileSystemRepository, Repository};
use crate::functions::Slug;
use color_eyre::eyre::Result;
use std::path::Path;

pub fn edit_with<E>(funky_dir: &Path, name: &str, editor_fn: E) -> Result<()>
where
  E: FnOnce(&str) -> Result<Option<String>>,
{
  let slug: Slug = name.parse()?;
  let repo = FileSystemRepository::new(funky_dir);
  let content = repo.read(&slug)?;

  match editor_fn(&content)? {
    Some(edited) => {
      repo.update(&slug, &edited)?;
      println!("Updated function: {}", slug);
    }
    None => {
      println!("Edit cancelled.");
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use tempfile::tempdir;

  #[test]
  fn test_edit_updates_function_content() {
    let tmp_dir = tempdir().unwrap();
    fs::write(tmp_dir.path().join("my-func.zsh"), "echo old").unwrap();

    edit_with(tmp_dir.path(), "my-func", |content| {
      assert_eq!(content, "echo old");
      Ok(Some(content.replace("echo old", "echo new")))
    })
    .unwrap();

    let result = fs::read_to_string(tmp_dir.path().join("my-func.zsh")).unwrap();
    assert_eq!(result, "echo new");
  }
}
