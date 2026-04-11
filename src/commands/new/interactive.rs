use crate::functions::repository::{FileSystemRepository, Repository};
use crate::functions::{zsh::Zsh, Function, FunctionSpec};
use color_eyre::eyre::{eyre, Result};
use inquire::InquireError;
use std::fs;
use std::path::Path;

use super::history;
use super::{read_clipboard, suggest_name};

#[derive(Debug, Clone)]
enum Source {
  History,
  Clipboard,
  TypeItIn,
}

impl std::fmt::Display for Source {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Source::History => write!(f, "History (browse recent commands)"),
      Source::Clipboard => write!(f, "Clipboard"),
      Source::TypeItIn => write!(f, "Type it in"),
    }
  }
}

fn cancelled() -> Result<()> {
  println!("Operation cancelled.");
  Ok(())
}

fn handle_prompt_error(err: InquireError) -> Result<()> {
  match err {
    InquireError::OperationCanceled | InquireError::OperationInterrupted => cancelled(),
    other => Err(other.into()),
  }
}

fn prompt_source() -> Result<Source> {
  let options = vec![Source::History, Source::Clipboard, Source::TypeItIn];
  let selection = inquire::Select::new("Where is your command?", options).prompt()?;
  Ok(selection)
}

fn prompt_command_from_history(history_file: &str) -> Result<String> {
  let path = crate::file::get_file(history_file.to_string())?;
  let contents = fs::read_to_string(path)?;
  let entries = history::parse_history(&contents);

  if entries.is_empty() {
    return Err(eyre!("No commands found in history file"));
  }

  let selection = inquire::Select::new("Select a command:", entries)
    .with_page_size(15)
    .with_help_message("↑↓ navigate, type to filter")
    .prompt()?;

  Ok(selection)
}

fn prompt_command_from_clipboard() -> Result<String> {
  let text = read_clipboard()?;
  println!("Found in clipboard: {}", text);
  let confirmed = inquire::Confirm::new("Use this command?")
    .with_default(true)
    .prompt()?;
  if confirmed {
    Ok(text)
  } else {
    Err(InquireError::OperationCanceled.into())
  }
}

fn prompt_command_typed() -> Result<String> {
  let text = inquire::Text::new("Enter your command:")
    .with_help_message("The shell command you want to turn into a function")
    .prompt()?;
  let trimmed = text.trim().to_string();
  if trimmed.is_empty() {
    return Err(eyre!("Command cannot be empty"));
  }
  Ok(trimmed)
}

fn prompt_name(command: &str) -> Result<String> {
  let default = suggest_name(command);
  let mut prompt = inquire::Text::new("Name this function:");
  if !default.is_empty() {
    prompt = prompt.with_default(&default);
  }
  let name = prompt.prompt()?;
  let trimmed = name.trim().to_string();
  if trimmed.is_empty() {
    return Err(eyre!("Function name cannot be empty"));
  }
  Ok(trimmed)
}

pub fn interactive_new(funky_dir: &Path, history_file: &str) -> Result<()> {
  match run_wizard(funky_dir, history_file) {
    Ok(()) => Ok(()),
    Err(err) => match err.downcast::<InquireError>() {
      Ok(inquire_err) => handle_prompt_error(inquire_err),
      Err(other) => Err(other),
    },
  }
}

fn run_wizard(funky_dir: &Path, history_file: &str) -> Result<()> {
  let source = prompt_source()?;

  let command = match source {
    Source::History => prompt_command_from_history(history_file)?,
    Source::Clipboard => prompt_command_from_clipboard()?,
    Source::TypeItIn => prompt_command_typed()?,
  };

  let tokens = super::tokenize_command(&command);
  let (command, arguments) = if tokens.len() > 1 {
    let selected =
      inquire::MultiSelect::new("Select tokens to make into function arguments:", tokens)
        .with_help_message("Space to select, Enter to confirm, or just Enter to skip")
        .prompt()?;

    if selected.is_empty() {
      (command, vec![])
    } else {
      let selected_refs: Vec<&super::CommandToken> = selected.iter().collect();
      super::replace_tokens_with_positional(&command, &selected_refs)
    }
  } else {
    (command, vec![])
  };

  let name = prompt_name(&command)?;

  let repo = FileSystemRepository::new(funky_dir);

  if repo.read(&name.parse()?).is_ok() {
    let overwrite = inquire::Confirm::new(&format!("'{}' already exists. Overwrite?", name))
      .with_default(false)
      .prompt()?;
    if !overwrite {
      return Err(InquireError::OperationCanceled.into());
    }
  }

  let spec = FunctionSpec::new(&name, command, arguments)?;
  let zsh = Zsh::new(repo);
  zsh.create(&spec)?;

  println!("Created function: {}", spec.name);

  Ok(())
}
