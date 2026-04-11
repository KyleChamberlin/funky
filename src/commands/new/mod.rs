use crate::file::get_file;
use crate::functions::{
  repository::FileSystemRepository, repository::Repository, zsh::Zsh, Function, FunctionSpec,
};
use color_eyre::eyre::{eyre, Result};
use std::fs;
use std::path::PathBuf;

pub mod history;
pub mod interactive;

pub use crate::args::{FunctionSource, NewArgs as Args};

#[derive(Debug, Clone)]
pub struct CommandToken {
  pub value: String,
  pub start: usize,
  pub end: usize,
}

impl std::fmt::Display for CommandToken {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

fn read_command_from_reader(reader: &mut dyn std::io::Read) -> Result<String> {
  let mut buffer = String::new();
  reader.read_to_string(&mut buffer)?;
  let trimmed = buffer.trim().to_string();
  if trimmed.is_empty() {
    return Err(eyre!("No input received from stdin"));
  }
  Ok(trimmed)
}

fn read_clipboard() -> Result<String> {
  let mut clipboard =
    arboard::Clipboard::new().map_err(|e| eyre!("Could not access clipboard: {}", e))?;
  let text = clipboard
    .get_text()
    .map_err(|e| eyre!("Could not read clipboard: {}", e))?;
  let trimmed = text.trim().to_string();
  if trimmed.is_empty() {
    return Err(eyre!("Clipboard is empty"));
  }
  Ok(trimmed)
}

pub fn suggest_name(command: &str) -> String {
  let skip = [
    "sudo", "env", "nohup", "nice", "time", "command", "builtin", "exec",
  ];
  command
    .split_whitespace()
    .find(|token| !skip.contains(token) && !token.starts_with('-') && !token.contains('='))
    .map(slug::slugify)
    .unwrap_or_default()
}

fn should_skip_token(token: &str) -> bool {
  let shell_operators = ["|", "&&", "||", ";", ">", ">>", "<", "2>", "&", "2>&1"];
  token.starts_with('-') || shell_operators.contains(&token) || token.contains('=')
}

fn sanitize_argument_name(token: &str) -> String {
  token
    .chars()
    .flat_map(|ch| ch.to_lowercase())
    .map(|ch| if ch.is_alphanumeric() { ch } else { '_' })
    .take(30)
    .collect()
}

pub fn tokenize_command(command: &str) -> Vec<CommandToken> {
  let mut tokens = Vec::new();
  let mut start = None;

  for (idx, ch) in command.char_indices() {
    if ch.is_whitespace() {
      if let Some(token_start) = start.take() {
        let value = &command[token_start..idx];
        if !should_skip_token(value) {
          tokens.push(CommandToken {
            value: value.to_string(),
            start: token_start,
            end: idx,
          });
        }
      }
    } else if start.is_none() {
      start = Some(idx);
    }
  }

  if let Some(token_start) = start {
    let value = &command[token_start..command.len()];
    if !should_skip_token(value) {
      tokens.push(CommandToken {
        value: value.to_string(),
        start: token_start,
        end: command.len(),
      });
    }
  }

  tokens
}

pub fn replace_tokens_with_positional(
  command: &str,
  selected: &[&CommandToken],
) -> (String, Vec<String>) {
  if selected.is_empty() {
    return (command.to_string(), vec![]);
  }

  let arguments = selected
    .iter()
    .map(|token| sanitize_argument_name(&token.value))
    .collect::<Vec<_>>();

  let mut replacements = selected
    .iter()
    .enumerate()
    .map(|(idx, token)| (*token, format!("${}", idx + 1)))
    .collect::<Vec<_>>();

  replacements.sort_by(|(left, _), (right, _)| right.start.cmp(&left.start));

  let mut modified = command.to_string();
  for (token, positional) in replacements {
    modified.replace_range(token.start..token.end, &positional);
  }

  (modified, arguments)
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
    FunctionSource::Clipboard => read_clipboard(),
    FunctionSource::Vargs => args
      .function
      .as_ref()
      .map(|s| s.join(" "))
      .ok_or_else(|| eyre!("No Vargs provided for SOURCE Vargs.")),
  }
}

pub fn new(funky_dir: &PathBuf, args: Args) -> Result<()> {
  let name = args
    .name
    .as_deref()
    .ok_or_else(|| eyre!("Function name is required. Run interactively or pass a name."))?;
  let command = get_command_from_source(&args)?;
  let spec = FunctionSpec::new(name, command, vec![])?;

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
      name: Some("test".to_string()),
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
      name: Some("my-func".to_string()),
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
      name: Some("my-func".to_string()),
      source: FunctionSource::Vargs,
      history_file: String::new(),
      overwrite: true,
      function: Some(vec!["echo".to_string(), "new".to_string()]),
    };

    let result = new(&tmp_dir.path().to_path_buf(), args);
    assert!(result.is_ok());
  }

  #[test]
  fn test_suggest_name_basic() {
    assert_eq!(suggest_name("docker compose up"), "docker");
    assert_eq!(suggest_name("echo hello world"), "echo");
  }

  #[test]
  fn test_suggest_name_skips_prefixes() {
    assert_eq!(suggest_name("sudo docker compose up"), "docker");
    assert_eq!(suggest_name("env FOO=bar python script.py"), "python");
    assert_eq!(suggest_name("nohup nice node server.js"), "node");
  }

  #[test]
  fn test_suggest_name_skips_flags_and_env_assigns() {
    assert_eq!(suggest_name("sudo -u root nginx -s reload"), "root");
  }

  #[test]
  fn test_suggest_name_empty() {
    assert_eq!(suggest_name(""), "");
    assert_eq!(suggest_name("   "), "");
  }

  #[test]
  fn test_tokenize_command_skips_flags() {
    let tokens = tokenize_command("docker compose up -d");
    let values: Vec<&str> = tokens.iter().map(|t| t.value.as_str()).collect();
    assert_eq!(values, vec!["docker", "compose", "up"]);
  }

  #[test]
  fn test_tokenize_command_skips_operators() {
    let tokens = tokenize_command("echo hello | grep world");
    let values: Vec<&str> = tokens.iter().map(|t| t.value.as_str()).collect();
    assert_eq!(values, vec!["echo", "hello", "grep", "world"]);
  }

  #[test]
  fn test_tokenize_command_skips_env_assignments() {
    let tokens = tokenize_command("FOO=bar python script.py");
    let values: Vec<&str> = tokens.iter().map(|t| t.value.as_str()).collect();
    assert_eq!(values, vec!["python", "script.py"]);
  }

  #[test]
  fn test_tokenize_command_preserves_offsets() {
    let cmd = "docker compose up";
    let tokens = tokenize_command(cmd);
    assert_eq!(tokens[2].value, "up");
    assert_eq!(&cmd[tokens[2].start..tokens[2].end], "up");
  }

  #[test]
  fn test_replace_tokens_single() {
    let cmd = "docker compose up";
    let tokens = tokenize_command(cmd);
    let up_token = tokens.iter().find(|t| t.value == "up").unwrap();
    let (modified, args) = replace_tokens_with_positional(cmd, &[up_token]);
    assert_eq!(modified, "docker compose $1");
    assert_eq!(args, vec!["up"]);
  }

  #[test]
  fn test_replace_tokens_multiple() {
    let cmd = "curl -X POST https://api.example.com/endpoint";
    let tokens = tokenize_command(cmd);
    let post = tokens.iter().find(|t| t.value == "POST").unwrap();
    let url = tokens
      .iter()
      .find(|t| t.value.starts_with("https"))
      .unwrap();
    let (modified, args) = replace_tokens_with_positional(cmd, &[post, url]);
    assert_eq!(modified, "curl -X $1 $2");
    assert_eq!(args, vec!["post", "https___api_example_com_endpoi"]);
  }

  #[test]
  fn test_replace_tokens_empty_selection() {
    let cmd = "echo hello";
    let (modified, args) = replace_tokens_with_positional(cmd, &[]);
    assert_eq!(modified, "echo hello");
    assert!(args.is_empty());
  }

  #[test]
  fn test_new_requires_name() {
    let tmp_dir = tempdir().unwrap();
    let args = Args {
      name: None,
      source: FunctionSource::Vargs,
      history_file: String::new(),
      overwrite: false,
      function: Some(vec!["echo".to_string(), "hi".to_string()]),
    };
    let result = new(&tmp_dir.path().to_path_buf(), args);
    assert!(result.is_err());
  }
}
