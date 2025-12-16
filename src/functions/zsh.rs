use super::{repository::Repository, Function, FunctionSpec};
use color_eyre::Result;
use tera::{Context, Tera};

pub struct Zsh<R: Repository> {
  tera: Tera,
  zsh_functions: R,
}

impl<R: Repository> Zsh<R> {
  pub fn new(repo: R) -> Self {
    let mut tera = Tera::default();
    tera
      .add_raw_template(
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
    self
      .tera
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
  use super::{super::repository::tests::MockRepository, *};

  #[test]
  fn test_zsh_create() {
    let repo = MockRepository::new();
    let zsh = Zsh::new(repo);
    let spec = FunctionSpec::new("Test Func", "echo 'hello world'".to_string(), vec![]).unwrap();

    zsh.create(&spec).unwrap();

    let functions = zsh.zsh_functions.functions.borrow();
    let created_function = functions.get("test-func").unwrap();

    assert!(created_function.contains("test-func"));
    assert!(created_function.contains("echo 'hello world'"));
  }
}
