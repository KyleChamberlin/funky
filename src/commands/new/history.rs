use std::collections::HashSet;

const HISTORY_LIMIT: usize = 200;

pub fn parse_history(contents: &str) -> Vec<String> {
  let mut entries = Vec::new();
  let mut current_extended: Option<String> = None;

  for line in contents.lines() {
    if let Some(command) = parse_extended_start(line) {
      push_trimmed(&mut entries, current_extended.take());
      current_extended = Some(command.to_string());
      continue;
    }

    if let Some(command) = current_extended.as_mut() {
      if command.ends_with('\\') || line.starts_with(char::is_whitespace) {
        command.push('\n');
        command.push_str(line);
        continue;
      }

      push_trimmed(&mut entries, current_extended.take());
    }

    push_trimmed(&mut entries, Some(line.to_string()));
  }

  push_trimmed(&mut entries, current_extended.take());

  let mut seen = HashSet::new();
  let mut deduped = Vec::with_capacity(HISTORY_LIMIT.min(entries.len()));

  for entry in entries.into_iter().rev() {
    if seen.insert(entry.clone()) {
      deduped.push(entry);
      if deduped.len() == HISTORY_LIMIT {
        break;
      }
    }
  }

  deduped
}

fn parse_extended_start(line: &str) -> Option<&str> {
  let remainder = line.strip_prefix(':')?.trim_start_matches(' ');
  let (timestamp, remainder) = remainder.split_once(':')?;
  let (duration, command) = remainder.split_once(';')?;

  if timestamp.is_empty()
    || duration.is_empty()
    || !timestamp.chars().all(|ch| ch.is_ascii_digit())
    || !duration.chars().all(|ch| ch.is_ascii_digit())
  {
    return None;
  }

  Some(command)
}

fn push_trimmed(entries: &mut Vec<String>, command: Option<String>) {
  if let Some(command) = command {
    let trimmed = command.trim();
    if !trimmed.is_empty() {
      entries.push(trimmed.to_string());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::parse_history;

  #[test]
  fn test_parse_plain_format() {
    let contents = "echo hello\nls -la\ngit status\n";

    assert_eq!(
      parse_history(contents),
      vec![
        "git status".to_string(),
        "ls -la".to_string(),
        "echo hello".to_string(),
      ]
    );
  }

  #[test]
  fn test_parse_extended_format() {
    let contents = ": 1234567890:0;echo hello\n: 1234567891:0;ls -la\n: 1234567892:0;git status\n";

    assert_eq!(
      parse_history(contents),
      vec![
        "git status".to_string(),
        "ls -la".to_string(),
        "echo hello".to_string(),
      ]
    );
  }

  #[test]
  fn test_parse_mixed_format() {
    let contents = "echo hello\n: 1234567891:0;ls -la\ngit status\n";

    assert_eq!(
      parse_history(contents),
      vec![
        "git status".to_string(),
        "ls -la".to_string(),
        "echo hello".to_string(),
      ]
    );
  }

  #[test]
  fn test_deduplication() {
    let contents = "echo hello\nls -la\necho hello\ngit status\nls -la\n";

    assert_eq!(
      parse_history(contents),
      vec![
        "ls -la".to_string(),
        "git status".to_string(),
        "echo hello".to_string(),
      ]
    );
  }

  #[test]
  fn test_ordering_most_recent_first() {
    let contents = "first\nsecond\nthird\n";

    assert_eq!(
      parse_history(contents),
      vec![
        "third".to_string(),
        "second".to_string(),
        "first".to_string(),
      ]
    );
  }

  #[test]
  fn test_empty_file() {
    assert!(parse_history("").is_empty());
  }

  #[test]
  fn test_whitespace_only() {
    assert!(parse_history("  \n\t\n  ").is_empty());
  }

  #[test]
  fn test_limit_200() {
    let contents = (0..300)
      .map(|i| format!("command-{i}"))
      .collect::<Vec<_>>()
      .join("\n");

    let parsed = parse_history(&contents);

    assert_eq!(parsed.len(), 200);
    assert_eq!(parsed.first(), Some(&"command-299".to_string()));
    assert_eq!(parsed.last(), Some(&"command-100".to_string()));
  }

  #[test]
  fn test_multiline_extended_command() {
    let contents = concat!(
      ": 1234567890:0;echo hello\\\n",
      "  world\n",
      ": 1234567891:0;ls -la\n"
    );

    assert_eq!(
      parse_history(contents),
      vec!["ls -la".to_string(), "echo hello\\\n  world".to_string(),]
    );
  }

  #[test]
  fn test_trims_commands_and_skips_empty_entries() {
    let contents = "  echo hello  \n\n: 1234567891:0;  ls -la  \n   \n";

    assert_eq!(
      parse_history(contents),
      vec!["ls -la".to_string(), "echo hello".to_string(),]
    );
  }
}
