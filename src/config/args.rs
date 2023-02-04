use clap::{Parser, Subcommand};

use crate::commands::new;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[clap()]
pub struct Args {
    #[arg(long, env, default_value = "~/.funky/")]
    pub funky_dir: String,

    #[command(subcommand)]
    pub command: Sub,
}

#[derive(Subcommand, Debug)]
pub enum Sub {
    Init {
        #[arg(default_value = "zsh")]
        shell: String,

        #[arg(long)]
        completion: bool,

        #[arg(long, default_value = "~/.zshrc")]
        rc_file: String,
    },
    New(new::Args),
    List,
    Edit,
}
