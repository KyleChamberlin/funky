use crate::file::get_file;
use crate::functions::{repository::FileSystemRepository, zsh::Zsh, Function, FunctionSpec};
use color_eyre::eyre::{eyre, Result};
use std::fs;
use std::path::PathBuf;

// Re-exporting Args and FunctionSource for main.rs
pub use crate::args::{FunctionSource, NewArgs as Args};

fn get_command_from_source(args: &Args) -> Result<String> {
  match args.source {
    FunctionSource::History => {
      let histfile_contents = fs::read_to_string(get_file(args.history_file.clone())?)?;
      histfile_contents
        .lines()
        .rev()
        .next()
        .map(String::from)
        .ok_or_else(|| eyre!("Unable to find command from HISTORY_FILE"))
    }
    FunctionSource::StdIn => todo!(),
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

  // For now, we'll hardcode Zsh. Later, this can come from config.
  let repo = FileSystemRepository::new(funky_dir);
  let zsh = Zsh::new(repo);
  zsh.create(&spec)?;

  println!("Created function: {}", spec.name);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::io::Write;
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
}
