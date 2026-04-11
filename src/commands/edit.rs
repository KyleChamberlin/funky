use crate::functions::repository::{FileSystemRepository, Repository};
use crate::functions::Slug;
use color_eyre::eyre::Result;
use std::env;
use std::fs as stdfs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

pub use crate::args::EditArgs;

fn resolve_editor(editor_override: Option<&str>) -> Result<String> {
  if let Some(editor) = editor_override {
    return Ok(editor.to_string());
  }
  if let Ok(editor) = env::var("VISUAL") {
    if !editor.is_empty() {
      return Ok(editor);
    }
  }
  if let Ok(editor) = env::var("EDITOR") {
    if !editor.is_empty() {
      return Ok(editor);
    }
  }
  for fallback in &["vim", "nano"] {
    if std::process::Command::new("which")
      .arg(fallback)
      .output()
      .map(|o| o.status.success())
      .unwrap_or(false)
    {
      return Ok(fallback.to_string());
    }
  }
  Err(color_eyre::eyre::eyre!(
    "No editor found. Set $VISUAL or $EDITOR, or install vim/nano."
  ))
}

fn open_in_editor(content: &str, editor_override: Option<&str>) -> Result<Option<String>> {
  let editor = resolve_editor(editor_override)?;

  let mut tmp_file = NamedTempFile::with_suffix(".zsh")?;
  tmp_file.write_all(content.as_bytes())?;
  tmp_file.flush()?;

  let tmp_path = tmp_file.into_temp_path();

  let status = Command::new(&editor).arg(&tmp_path).status()?;

  if !status.success() {
    return Err(color_eyre::eyre::eyre!(
      "Editor '{}' exited with error",
      editor
    ));
  }

  let edited = stdfs::read_to_string(&tmp_path)?;

  if edited == content {
    Ok(None)
  } else {
    Ok(Some(edited))
  }
}

pub fn edit(funky_dir: &Path, args: EditArgs) -> Result<()> {
  let editor_override = args.editor.as_deref();
  edit_with(funky_dir, &args.name, |content| {
    open_in_editor(content, editor_override)
  })
}

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

  #[test]
  fn test_edit_cancelled_leaves_function_unchanged() {
    let tmp_dir = tempdir().unwrap();
    fs::write(tmp_dir.path().join("my-func.zsh"), "echo original").unwrap();

    edit_with(tmp_dir.path(), "my-func", |_| Ok(None)).unwrap();

    let result = fs::read_to_string(tmp_dir.path().join("my-func.zsh")).unwrap();
    assert_eq!(result, "echo original");
  }

  #[test]
  fn test_edit_nonexistent_function_errors() {
    let tmp_dir = tempdir().unwrap();
    let result = edit_with(tmp_dir.path(), "nope", |_| Ok(Some("x".into())));
    assert!(result.is_err());
  }
}
