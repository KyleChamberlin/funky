use color_eyre::eyre::{eyre, Result};
use serde::Serialize;
use slug::slugify;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

pub mod repository;
pub mod zsh;

#[derive(Serialize, Debug)]
pub struct Slug(String);

impl FromStr for Slug {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slug = slugify(s);
        if slug.is_empty() {
            return Err(eyre!("slug for '{}' cannot be empty", s));
        }
        Ok(Slug(slug))
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Debug)]
pub struct FunctionSpec {
    pub name: Slug,
    pub command: String,
    pub arguments: Vec<String>,
}

impl FunctionSpec {
    pub fn new(name: &str, command: String, arguments: Vec<String>) -> Result<Self> {
        Ok(Self {
            name: name.parse()?,
            command,
            arguments,
        })
    }
}

pub trait Function {
    fn create(&self, spec: &FunctionSpec, funky_dir: &PathBuf) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_slug_from_str() {
        assert_eq!(Slug::from_str("Hello World").unwrap().0, "hello-world");
        assert_eq!(Slug::from_str("foo-bar").unwrap().0, "foo-bar");
        assert_eq!(Slug::from_str("  leading").unwrap().0, "leading");
        assert_eq!(Slug::from_str("trailing  ").unwrap().0, "trailing");
        assert_eq!(Slug::from_str("special!@#$").unwrap().0, "special");
        assert!(Slug::from_str("").is_err());
        assert!(Slug::from_str(" ").is_err());
        assert!(Slug::from_str("!@#$").is_err());
    }
}