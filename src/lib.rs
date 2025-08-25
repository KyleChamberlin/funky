use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use tera::Tera;

pub mod args;
pub mod commands;
pub mod file;
pub mod functions;

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        for file in TEMPLATE_DIR.files() {
            let path = file.path().to_str().unwrap();
            let contents = std::str::from_utf8(file.contents()).unwrap();
            tera.add_raw_template(path, contents).unwrap();
        }
        tera
    };
}
