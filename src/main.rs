use std::{fs, path::PathBuf};

use clap::Parser;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use shellexpand;
use tera::{Context, Tera};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, env, default_value = "~/.zsh_history")]
    histfile: PathBuf,

    #[arg(long, env, default_value = "~/.funky/")]
    funky_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Func {
    name: String,
    arguments: Vec<String>,
    command: String,
}

fn main() {
    ctrlc::set_handler(move || {
        println!("Exiting early.");
    })
    .expect("Error setting Ctrl-C handler");

    let _args = Args::parse();

    let histfile_contents =
        fs::read_to_string(_args.histfile.as_path()).expect("Histfile should exist");

    let last_entry = histfile_contents.lines().rev().nth(1).unwrap();

    let func = Func {
        name: "cmd".to_string(),
        arguments: vec!["arg1".to_string(), "arg2".to_string()],
        command: last_entry.to_string(),
    };

    let _function_out = TERA
        .render(
            "functions/zsh",
            &Context::from_serialize(func).expect("failed to serialize context"),
        )
        .expect("failed to render template into string");

    let funky_dir = get_dir(_args.funky_dir);

    dbg!(funky_dir);
}

fn get_dir(path: String) -> PathBuf {
    let expanded_path = shellexpand::full(&path).expect("Failed to expand path.");
    let dir = PathBuf::from(expanded_path.to_string());

    if dir.exists() {
        if !dir.is_dir() {
            panic!("provided path is not a directory.");
        }
    } else {
        fs::create_dir_all(&dir).expect("unable to create dir");
    };

    return dir;
}

lazy_static! {
    pub static ref TERA: Tera = {
        let tera = match Tera::new("template/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(127);
            }
        };
        tera
    };
}
