use clap::{arg, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[clap()]
pub struct Args {
  #[arg(long, env, default_value = "~/.funky/")]
  pub funky_dir: String,

  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  Init {
    #[arg(default_value = "zsh")]
    shell: String,

    #[arg(long)]
    completion: bool,

    #[arg(long, default_value = "~/.zshrc")]
    rc_file: String,
  },
  New(NewArgs),
  List,
  Edit,
}

#[derive(Debug, Parser)]
pub struct NewArgs {
  #[arg()]
  pub name: String,

  #[arg(value_enum, long = "from", default_value = "vargs")]
  pub source: FunctionSource,

  /// History files are currently only supported if in a format without extra metadata
  /// TODO: add parsing rules for extended format history
  #[arg(long, env = "HISTFILE", default_value = "~/.zsh_history")]
  pub history_file: String,

  /// if another function is found in FUNKY_DIR with the same NAME,
  /// overwrite the contents of that function without prompting.
  #[arg(long)]
  pub overwrite: bool,

  /// The command you wish to make funky.
  /// If your command includes shell interpreted glyphs you will need to either
  /// escape them or quote your command to stop shell interpretation.
  #[arg(name = "vargs", last = true)]
  pub function: Option<Vec<String>>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum FunctionSource {
  History,
  StdIn,
  Clipboard,
  Vargs,
}
