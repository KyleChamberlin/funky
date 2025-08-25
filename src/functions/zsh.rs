use super::Function;
use color_eyre::Result;
use std::path::PathBuf;

pub struct Zsh;

impl Function for Zsh {
    fn create(&self, _funky_dir: &PathBuf, _name: &str, _body: &str) -> Result<()> {
        unimplemented!()
    }
}
