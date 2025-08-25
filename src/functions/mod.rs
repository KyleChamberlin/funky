use color_eyre::Result;
use std::path::PathBuf;

pub mod zsh;

pub trait Function {
    fn create(&self, funky_dir: &PathBuf, name: &str, body: &str) -> Result<()>;
}
