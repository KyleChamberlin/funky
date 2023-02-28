use std::fs::OpenOptions;
use std::io::Write;
use std::{
  fs::{self},
  path::PathBuf,
};

use clap::{arg, Parser, ValueEnum};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{file::get_file, TEMPLATES};

#[derive(Debug, Parser)]
pub struct Args {
  #[arg()]
  name: String,

  #[arg(value_enum, long = "from", default_value = "vargs")]
  source: FunctionSource,

  /// History files are currently only supported if in a format without extra metadata
  /// TODO: add parsing rules for extended format history
  #[arg(long, env = "HISTFILE", default_value = "~/.zsh_history")]
  history_file: String,

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
pub enum FunctionSource {
  History,
  StdIn,
  Clipboard,
  Vargs,
}

#[derive(Serialize, Deserialize, Debug)]
struct Func {
  name: String,
  arguments: Vec<String>,
  command: String,
}

pub fn new(funky_dir: PathBuf, args: Args) -> Result<()> {
  let command = match args.source {
    FunctionSource::History => {
      let history_contents = fs::read_to_string(get_file(args.history_file)?)?;

      match history_contents.lines().rev().nth(1) {
        Some(s) => Ok(s.to_string()),
        None => Err(eyre!("Unable to find command from HISTORY_FILE")),
      }
    }
    FunctionSource::StdIn => todo!(),
    FunctionSource::Clipboard => todo!(),
    FunctionSource::Vargs => match args.function {
      Some(s) => Ok(s.join(" ")),
      None => Err(eyre!("No Vargs provided for SOURCE Vargs.")),
    },
  }?;
  
  let mut parts: Vec<&str> = vec![];
  for part in command.split(" ") {
    parts.push(part);
  }
  
  dbg!(&parts);

  let func = Func {
    name: args.name.clone(),
    arguments: vec![],
    command,
  };

  let function_path = funky_dir.join(args.name);

  let _function_out = TEMPLATES.render("functions/zsh", &Context::from_serialize(&func)?)?;

  match OpenOptions::new()
    .write(true)
    .create(true)
    .open(function_path)?
    .write_all(_function_out.as_ref())
  {
    Ok(..) => Ok(()),
    Err(e) => Err(eyre!(e)),
  }
}
