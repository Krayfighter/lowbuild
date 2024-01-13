
use super::Target;

use anyhow::Result;


#[derive(Debug)]
pub struct CargoTarget {
    name: String,
    deps: Vec<String>,
    base_dir: std::path::PathBuf,
    // link_dirs: Vec<std::path::PathBuf>,
    // include_dirs: Vec<std::path::PathBuf>,
}

impl CargoTarget {
    pub fn from_config(
        config: crate::config::CargoTargetConfig,
        name: String,
    ) -> Self {
        return Self {
            name,
            deps: config.deps.unwrap_or_default(),
            base_dir: config.dir,
        }
    }
}

impl Target for CargoTarget {
    fn name(&self) -> String { return self.name.clone(); }
    fn deps(&self) -> Vec<String> { return self.deps.clone(); }
    fn should_recompile(&self) -> bool {todo!();}
    fn compile(&self) -> Result<()> {
        // let mut command_buffer = String::from("cd ") + &self.base_dir.into_os_string().into_string().unwrap();
        // command_buffer += " && cargo build"
        std::process::Command::new("cargo")
            .current_dir(self.base_dir.clone())
            .arg("build")
            .status()?;
        return Ok(());
    }
}

