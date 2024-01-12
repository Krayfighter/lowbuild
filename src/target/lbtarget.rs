
use std::io::Read;

use anyhow::Result;

use super::Target;





fn find_sources(
  path: std::path::PathBuf,
  pattern: &str,
) -> Result<Vec<std::path::PathBuf>> {
  // let query_string = String::from("find . ")
    // + path.as_os_str().to_string_lossy().as_ref()
    // + " -name \""
    // + pattern;
    // + "\"";
  let finder = std::process::Command::new("sh")
    .arg("-c")
    .arg(String::from("find . -name \"") + pattern + "\"")
    .current_dir(path)
    .stdout(std::process::Stdio::piped())
    .spawn()?;

  let mut buffer = String::new();
  finder.stdout.ok_or(
    anyhow!("unable to read stdout of child process")
  )?.read_to_string(&mut buffer)?;

  return Ok( buffer.lines()
    .map(|line| std::path::PathBuf::from(line))
    .collect::<Vec<std::path::PathBuf>>() );
}


#[derive(Debug, Default)]
pub struct LBCTarget {
  id: uuid::Uuid,
  deps: Vec<uuid::Uuid>,
  base_dir: std::path::PathBuf,
  sources: Vec<std::path::PathBuf>,
  link_dirs: Vec<std::path::PathBuf>,
  pub include_dirs: Vec<std::path::PathBuf>,
  compiler: String,
}

impl LBCTarget {
  pub fn new(base_dir: String, compiler: String) -> Result<Self> {
    let id = uuid::Uuid::new_v4();
    let base_dir = std::path::PathBuf::from(base_dir);
    std::fs::try_exists(&base_dir).expect("`base_dir1 doesn't exist`");
    let c_sources = find_sources(base_dir.clone(), "*.c").expect("unable to list source files");
    return Ok( Self {
      id,
      base_dir,
      sources: c_sources,
      compiler,
      ..Default::default()
    } );
  }

  fn concat_symbol(symbol: String, vector: &Vec<std::path::PathBuf>) -> String {
    return vector.iter()
      .map(|path| symbol.clone() + path.to_string_lossy().as_ref())
      .fold(String::new(), |acc, path| {
        return acc + path.as_str();
      });
  }

  pub fn concat_link_dirs(&self) -> String {
    return Self::concat_symbol(" -L".to_string(), &self.link_dirs);
  }

  pub fn concat_include_dirs(&self) -> String {
    return Self::concat_symbol(" -I".to_string(), &self.include_dirs);
  }

  pub fn sources_string(&self) -> String {
    return self.sources.iter()
      .map(|source_dir| source_dir.to_string_lossy().as_ref().to_string())
      .fold(String::new(), |acc, source| {
        acc + " " + source.as_str()
      });
  }
}

impl super::Target for LBCTarget {
    fn dependencies(&self) -> Vec<uuid::Uuid> { return self.deps.clone(); }

    fn uuid(&self) -> uuid::Uuid { return self.id; }

    fn should_recompile(&self) -> bool { todo!() }
    fn add_link_dir(&mut self, dir: std::path::PathBuf) {
        self.link_dirs.push(dir);
    }
    fn add_include_dir(&mut self, dir: std::path::PathBuf) {
        self.include_dirs.push(dir);
    }

    fn compile(&self) -> Result<()> {
      // println!("current dir: {:?}", &self.base_dir);
      // let dir = std::process::Command::new("pwd")
      //   .current_dir(&self.base_dir)
      //   .output()?;
      // println!("{:?}", dir);
      // println!("LBCTarget DBG: {:?}", self);
      // println!("include dirs (compile): {:?}", &self.include_dirs);
      // println!("concat incvlude dirs: {}", self.concat_include_dirs());
      let cc_command = self.compiler.clone()
        + &self.concat_link_dirs()
        + &self.concat_include_dirs()
        + &self.sources_string();

      println!("C build command: {}", &cc_command);
      
      let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg(cc_command)
        // .arg(self.compiler.clone())
        // .arg(self.sources_string())
        // .arg(self.concat_link_dirs())
        // .arg(self.concat_include_dirs())
        .current_dir(self.base_dir.clone())
        .status()?;
        
        return Ok(());
    }
}


