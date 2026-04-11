use crate::file::get_file;
use crate::functions::{repository::FileSystemRepository, repository::Repository, zsh::Zsh, Function, FunctionSpec};
use color_eyre::eyre::{eyre, Result};
use std::fs;
use std::path::PathBuf;

// Re-exporting Args and FunctionSource for main.rs
pub use crate::args::{FunctionSource, NewArgs as Args};

fn read_command_from_reader(reader: &mut dyn std::io::Read) -> Result<String> {
  let mut buffer = String::new();
  reader.read_to_string(&mut buffer)?;
  let trimmed = buffer.trim().to_string();
  if trimmed.is_empty() {
    return Err(eyre!("No input received from stdin"));
  }
  Ok(trimmed)
}

fn get_command_from_source(args: &Args) -> Result<String> {
  match args.source {
    FunctionSource::History => {
      let histfile_contents = fs::read_to_string(get_file(args.history_file.clone())?)?;
      histfile_contents
        .lines()
        .last()
        .map(String::from)
        .ok_or_else(|| eyre!("Unable to find command from HISTORY_FILE"))
    }
    FunctionSource::StdIn => read_command_from_reader(&mut std::io::stdin()),
    FunctionSource::Clipboard => todo!(),
    FunctionSource::Vargs => args
      .function
      .as_ref()
      .map(|s| s.join(" "))
      .ok_or_else(|| eyre!("No Vargs provided for SOURCE Vargs.")),
  }
}

pub fn new(funky_dir: &PathBuf, args: Args) -> Result<()> {
  let command = get_command_from_source(&args)?;
  let spec = FunctionSpec::new(&args.name, command, vec![])?;

  let repo = FileSystemRepository::new(funky_dir);

  if !args.overwrite && repo.read(&spec.name).is_ok() {
    return Err(eyre!(
      "Function '{}' already exists. Use --overwrite to replace it.",
      spec.name
    ));
  }

  let zsh = Zsh::new(repo);
  zsh.create(&spec)?;

  println!("Created function: {}", spec.name);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::io::{Cursor, Write};
  use tempfile::tempdir;

  #[test]
  fn test_get_command_from_history() {
    let tmp_dir = tempdir().unwrap();
    let history_file_path = tmp_dir.path().join(".zsh_history");
    let mut history_file = File::create(&history_file_path).unwrap();
    writeln!(history_file, "echo hello").unwrap();
    writeln!(history_file, "echo world").unwrap();

    let args = Args {
      name: "test".to_string(),
      source: FunctionSource::History,
      history_file: history_file_path.to_str().unwrap().to_string(),
      overwrite: false,
      function: None,
    };

    let result = get_command_from_source(&args).unwrap();
    assert_eq!(result, "echo world");
  }

  #[test]
  fn test_read_command_from_reader() {
    let mut reader = Cursor::new(b"echo hello world\n");
    let result = read_command_from_reader(&mut reader).unwrap();
    assert_eq!(result, "echo hello world");
  }

  #[test]
  fn test_new_rejects_duplicate_without_overwrite() {
    let tmp_dir = tempdir().unwrap();
    fs::write(tmp_dir.path().join("my-func.zsh"), "echo old").unwrap();

    let args = Args {
      name: "my-func".to_string(),
      source: FunctionSource::Vargs,
      history_file: String::new(),
      overwrite: false,
      function: Some(vec!["echo".to_string(), "new".to_string()]),
    };

    let result = new(&tmp_dir.path().to_path_buf(), args);
    assert!(result.is_err());
  }

  #[test]
  fn test_new_allows_overwrite_when_flag_set() {
    let tmp_dir = tempdir().unwrap();
    fs::write(tmp_dir.path().join("my-func.zsh"), "echo old").unwrap();

    let args = Args {
      name: "my-func".to_string(),
      source: FunctionSource::Vargs,
      history_file: String::new(),
      overwrite: true,
      function: Some(vec!["echo".to_string(), "new".to_string()]),
    };

    let result = new(&tmp_dir.path().to_path_buf(), args);
    assert!(result.is_ok());
  }
}
