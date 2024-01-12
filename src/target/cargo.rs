
use super::Target;

use anyhow::Result;


#[derive(Debug)]
pub struct CargoTarget {
    // deps: Vec<Target>,
    id: uuid::Uuid,
    deps: Vec<uuid::Uuid>,
    base_dir: std::path::PathBuf,
    // link_dirs: Vec<std::path::PathBuf>,
    // include_dirs: Vec<std::path::PathBuf>,
}

impl CargoTarget {
    pub fn new(base_dir: String) -> Result<Self> {
        let id = uuid::Uuid::new_v4();
        let base_dir = std::path::PathBuf::from(base_dir);
        std::fs::try_exists(&base_dir)?;

        return Ok( Self {
            id,
            deps: vec!(),
            base_dir,
        } );
    }
}

impl Target for CargoTarget {
    fn dependencies(&self) -> Vec<uuid::Uuid> {
        return self.deps.clone();
    }

    fn uuid(&self) -> uuid::Uuid { return self.id; }

    fn should_recompile(&self) -> bool {todo!();}
    fn add_link_dir(&mut self, dir: std::path::PathBuf) {todo!();}
    fn add_include_dir(&mut self, dir: std::path::PathBuf) {todo!();}

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

