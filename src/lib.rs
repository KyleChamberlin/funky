use lazy_static::lazy_static;
use tera::Tera;

pub mod commands;
pub mod config;
pub mod file;

lazy_static! {
  pub static ref TEMPLATES: Tera = {
    match Tera::new("template/**/*") {
      Ok(t) => t,
      Err(e) => {
        println!("Parsing error(s): {e}");
        ::std::process::exit(127);
      }
    }
  };
}
