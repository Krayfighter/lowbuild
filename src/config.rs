
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct BuildConfig {
  pub config: Option<CompilerConfig>,
  pub targets: std::collections::HashMap<String, TargetConfig>
}

#[derive(Deserialize, Debug)]
pub enum TargetConfig {
  RustCargo(CargoTargetConfig),
  CLowBuild(CFamilyConfig),
}

#[derive(Deserialize, Debug)]
pub struct CargoTargetConfig {
  pub deps: Option<Vec<String>>,
  pub dir: PathBuf,
  // build: RustBuildType,
}

#[derive(Deserialize, Debug)]
pub struct CFamilyConfig {
  pub deps: Option<Vec<String>>,
  pub dir: PathBuf,
  // build: CFamilyBuildType,
  pub links: Option<Vec<PathBuf>>,
  pub includes: Option<Vec<PathBuf>>,
  pub sources: Option<SourcesList>,
  pub compiler: CompilerOption,
}

#[derive(Deserialize, Debug)]
pub enum SourcesList {
  Auto,
  Files(Vec<std::path::PathBuf>),
}

#[derive(Deserialize, Debug, Clone)]
pub enum CompilerOption {
  CC,
  CXX,
  Custom(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct CompilerConfig {
  pub cc: Option<String>,
  pub cxx: Option<String>,
}

// #[derive(Deserialize, Debug)]
// pub struct DirvList (pub Vec<PathBuf>);


