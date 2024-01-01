use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Args {
  #[arg()]
  name: String,

  #[arg(value_enum, default_value = "fish")]
  shell: Shell,

  /// if another function is found in FUNKY_DIR with the same NAME,
  /// overwrite the contents of that function without prompting.
  #[arg(long)]
  overwrite: bool,

  /// The command you wish to make funky.
  /// If your command includes shell interpreted glyphs you will need to either
  /// escape them or quote your command to stop shell interpretation.
  #[arg(name = "vargs", last = true)]
  function: Option<Vec<String>>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Shell {
  Fish,
  Zsh,
  Bash,
  Powershell,
}
