use super::{Function, FunctionSpec};
use color_eyre::Result;
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

pub struct Zsh {
    tera: Tera,
}

impl Default for Zsh {
    fn default() -> Self {
        let mut tera = Tera::default();
        tera.add_raw_template(
            "functions/zsh",
            include_str!("../../template/functions/zsh.tera"),
        )
        .unwrap();
        Self { tera }
    }
}

impl Function for Zsh {
    fn create(&self, spec: &FunctionSpec, funky_dir: &PathBuf) -> Result<()> {
        let function_out =
            self.tera
                .render("functions/zsh", &Context::from_serialize(spec)?)?;
        let file_path = funky_dir.join(format!("{}.zsh", spec.name));
        fs::write(file_path, function_out)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_zsh_create() {
        let tmp_dir = tempdir().unwrap();
        let funky_dir = tmp_dir.path().to_path_buf();
        let zsh = Zsh::default();
        let spec =
            FunctionSpec::new("Test Func", "echo 'hello world'".to_string(), vec![]).unwrap();

        zsh.create(&spec, &funky_dir).unwrap();

        let file_path = funky_dir.join("test-func.zsh");
        assert!(file_path.exists());
        let content = fs::read_to_string(file_path).unwrap();
        assert!(content.contains("test-func"));
        assert!(content.contains("echo 'hello world'"));
    }
}