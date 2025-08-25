use super::{
    repository::Repository,
    Function, FunctionSpec,
};
use color_eyre::Result;
use tera::{Context, Tera};

pub struct Zsh<R: Repository> {
    tera: Tera,
    zsh_functions: R,
}

impl<R: Repository> Zsh<R> {
    pub fn new(repo: R) -> Self {
        let mut tera = Tera::default();
        tera.add_raw_template(
            "functions/zsh",
            include_str!("../../template/functions/zsh.tera"),
        )
        .unwrap();
        Self {
            tera,
            zsh_functions: repo,
        }
    }

    fn render_body(&self, spec: &FunctionSpec) -> Result<String> {
        self.tera
            .render("functions/zsh", &Context::from_serialize(spec)?)
            .map_err(Into::into)
    }
}

impl<R: Repository> Function for Zsh<R> {
    fn create(&self, spec: &FunctionSpec) -> Result<()> {
        let body = self.render_body(spec)?;
        self.zsh_functions.create(&spec.name, &body)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::repository::FileSystemRepository,
        *,
    };
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_zsh_create() {
        let tmp_dir = tempdir().unwrap();
        let repo = FileSystemRepository::new(tmp_dir.path());
        let zsh = Zsh::new(repo);
        let spec =
            FunctionSpec::new("Test Func", "echo 'hello world'".to_string(), vec![]).unwrap();

        zsh.create(&spec).unwrap();

        let file_path = tmp_dir.path().join("test-func.zsh");
        assert!(file_path.exists());
        let content = fs::read_to_string(file_path).unwrap();
        assert!(content.contains("test-func"));
        assert!(content.contains("echo 'hello world'"));
    }
}
