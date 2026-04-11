use std::io::IsTerminal;

use clap::Parser;
use color_eyre::eyre::eyre;
use color_eyre::Result;

use funky_lib::args::{Args, Command};
use funky_lib::commands;
use funky_lib::file::get_dir;

fn setup_cli_nice_to_haves() -> Result<()> {
  color_eyre::install()?;

  ctrlc::set_handler(move || {
    println!("Ctrl+C recieved, terminating.");
  })?;

  Ok(())
}

fn main() -> Result<()> {
  setup_cli_nice_to_haves()?;

  let args = Args::parse();

  let funky_dir = get_dir(args.funky_dir)?;

  match args.command {
    Command::New(ref function_args) if function_args.name.is_none() => {
      if !std::io::stdin().is_terminal() {
        return Err(eyre!(
          "Missing required argument: NAME\n\nUsage: funky new <NAME> [-- <COMMAND>...]\n\nFor interactive mode, run in a terminal."
        ));
      }
      commands::new::interactive::interactive_new(&funky_dir, &function_args.history_file)
    }
    Command::New(function_args) => commands::new::new(&funky_dir, function_args),
    Command::List => commands::list::list(&funky_dir),
    Command::Init { shell, rc_file, .. } => commands::init::init(&funky_dir, &shell, &rc_file),
    Command::Edit(ref edit_args) if edit_args.name.is_none() => {
      if !std::io::stdin().is_terminal() {
        return Err(eyre!(
          "Missing required argument: NAME\n\nUsage: funky edit <NAME>\n\nFor interactive mode, run in a terminal."
        ));
      }
      commands::edit::interactive_edit(&funky_dir, edit_args.editor.clone())
    }
    Command::Edit(edit_args) => commands::edit::edit(&funky_dir, edit_args),
  }
}
