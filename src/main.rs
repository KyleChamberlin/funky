use clap::Parser;
use color_eyre::Result;

use funky_lib::commands;
use funky_lib::config::{Args, Command};
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
    Command::New(function) => commands::new::new(&funky_dir, function),
    _ => todo!(),
  }
}
