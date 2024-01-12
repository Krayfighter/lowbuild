
use anyhow::Result;

pub mod cargo;
pub mod lbtarget;

pub trait Target: std::fmt::Debug {
    fn dependencies(&self) -> Vec<uuid::Uuid>;
    fn uuid(&self) -> uuid::Uuid;
    fn should_recompile(&self) -> bool;
    fn add_link_dir(&mut self, dir: std::path::PathBuf);
    fn add_include_dir(&mut self, dir: std::path::PathBuf);
    fn compile(&self) -> Result<()>;
}

