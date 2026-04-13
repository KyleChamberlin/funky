use color_eyre::eyre::{Result, eyre};
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

pub fn init(funky_dir: &Path, shell: &str, rc_file: &str) -> Result<()> {
  if shell != "zsh" {
    return Err(eyre!(
      "Unsupported shell: {}. Only zsh is currently supported.",
      shell
    ));
  }

  let rc_path = PathBuf::from(shellexpand::full(rc_file)?.to_string());

  let mut tera = Tera::default();
  tera.add_raw_template("zsh/config", include_str!("../../template/zsh/config"))?;

  let mut context = Context::new();
  context.insert("funky_dir", &funky_dir.display().to_string());

  let snippet = tera.render("zsh/config", &context)?;

  let existing_contents = if rc_path.exists() {
    fs::read_to_string(&rc_path)?
  } else {
    String::new()
  };

  if existing_contents.contains(snippet.trim()) {
    println!("Shell already configured for funky.");
    return Ok(());
  }

  let mut new_contents = existing_contents;
  if !new_contents.ends_with('\n') && !new_contents.is_empty() {
    new_contents.push('\n');
  }
  new_contents.push_str(&snippet);

  fs::write(&rc_path, new_contents)?;

  println!("Updated {} for funky.", rc_file);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use tempfile::tempdir;

  #[test]
  fn test_init_creates_config() {
    let tmp_dir = tempdir().unwrap();
    let funky_dir = tmp_dir.path().join("funky");
    fs::create_dir_all(&funky_dir).unwrap();
    let rc_file = tmp_dir.path().join(".zshrc");

    init(&funky_dir, "zsh", rc_file.to_str().unwrap()).unwrap();

    let contents = fs::read_to_string(&rc_file).unwrap();
    assert!(contents.contains("fpath=("));
    assert!(contents.contains("autoload -Uz"));
  }

  #[test]
  fn test_init_idempotent() {
    let tmp_dir = tempdir().unwrap();
    let funky_dir = tmp_dir.path().join("funky");
    fs::create_dir_all(&funky_dir).unwrap();
    let rc_file = tmp_dir.path().join(".zshrc");

    init(&funky_dir, "zsh", rc_file.to_str().unwrap()).unwrap();
    init(&funky_dir, "zsh", rc_file.to_str().unwrap()).unwrap();

    let contents = fs::read_to_string(&rc_file).unwrap();
    let count = contents.matches("autoload -Uz").count();
    assert_eq!(count, 1, "Config should only appear once");
  }

  #[test]
  fn test_init_appends_to_existing() {
    let tmp_dir = tempdir().unwrap();
    let funky_dir = tmp_dir.path().join("funky");
    fs::create_dir_all(&funky_dir).unwrap();
    let rc_file = tmp_dir.path().join(".zshrc");
    fs::write(&rc_file, "# existing config\nexport FOO=bar\n").unwrap();

    init(&funky_dir, "zsh", rc_file.to_str().unwrap()).unwrap();

    let contents = fs::read_to_string(&rc_file).unwrap();
    assert!(contents.starts_with("# existing config"));
    assert!(contents.contains("autoload -Uz"));
  }

  #[test]
  fn test_init_unsupported_shell() {
    let tmp_dir = tempdir().unwrap();
    let funky_dir = tmp_dir.path().join("funky");
    let rc_file = tmp_dir.path().join(".bashrc");

    let result = init(&funky_dir, "bash", rc_file.to_str().unwrap());
    assert!(result.is_err());
    assert!(
      result
        .unwrap_err()
        .to_string()
        .contains("Unsupported shell")
    );
  }
}
