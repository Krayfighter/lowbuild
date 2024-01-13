
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


// #[derive(Debug, Default)]
// pub struct LBCTarget {
//   id: uuid::Uuid,
//   deps: Vec<uuid::Uuid>,
//   base_dir: std::path::PathBuf,
//   sources: Vec<std::path::PathBuf>,
//   link_dirs: Vec<std::path::PathBuf>,
//   pub include_dirs: Vec<std::path::PathBuf>,
//   compiler: String,
// }

// impl LBCTarget {
//   pub fn new(base_dir: String, compiler: String) -> Result<Self> {
//     let id = uuid::Uuid::new_v4();
//     let base_dir = std::path::PathBuf::from(base_dir);
//     std::fs::try_exists(&base_dir).expect("`base_dir1 doesn't exist`");
//     let c_sources = find_sources(base_dir.clone(), "*.c").expect("unable to list source files");
//     return Ok( Self {
//       id,
//       base_dir,
//       sources: c_sources,
//       compiler,
//       ..Default::default()
//     } );
//   }

//   fn concat_symbol(symbol: String, vector: &Vec<std::path::PathBuf>) -> String {
//     return vector.iter()
//       .map(|path| symbol.clone() + path.to_string_lossy().as_ref())
//       .fold(String::new(), |acc, path| {
//         return acc + path.as_str();
//       });
//   }

//   pub fn concat_link_dirs(&self) -> String {
//     return Self::concat_symbol(" -L".to_string(), &self.link_dirs);
//   }

//   pub fn concat_include_dirs(&self) -> String {
//     return Self::concat_symbol(" -I".to_string(), &self.include_dirs);
//   }

//   pub fn sources_string(&self) -> String {
//     return self.sources.iter()
//       .map(|source_dir| source_dir.to_string_lossy().as_ref().to_string())
//       .fold(String::new(), |acc, source| {
//         acc + " " + source.as_str()
//       });
//   }
// }

// impl super::Target for LBCTarget {
//     fn dependencies(&self) -> Vec<uuid::Uuid> { return self.deps.clone(); }

//     fn uuid(&self) -> uuid::Uuid { return self.id; }

//     fn should_recompile(&self) -> bool { todo!() }
//     fn add_link_dir(&mut self, dir: std::path::PathBuf) {
//         self.link_dirs.push(dir);
//     }
//     fn add_include_dir(&mut self, dir: std::path::PathBuf) {
//         self.include_dirs.push(dir);
//     }

//     fn compile(&self) -> Result<()> {
//       // println!("current dir: {:?}", &self.base_dir);
//       // let dir = std::process::Command::new("pwd")
//       //   .current_dir(&self.base_dir)
//       //   .output()?;
//       // println!("{:?}", dir);
//       // println!("LBCTarget DBG: {:?}", self);
//       // println!("include dirs (compile): {:?}", &self.include_dirs);
//       // println!("concat incvlude dirs: {}", self.concat_include_dirs());
//       let cc_command = self.compiler.clone()
//         + &self.concat_link_dirs()
//         + &self.concat_include_dirs()
//         + &self.sources_string();

//       println!("C build command: {}", &cc_command);
      
//       let _ = std::process::Command::new("sh")
//         .arg("-c")
//         .arg(cc_command)
//         // .arg(self.compiler.clone())
//         // .arg(self.sources_string())
//         // .arg(self.concat_link_dirs())
//         // .arg(self.concat_include_dirs())
//         .current_dir(self.base_dir.clone())
//         .status()?;
        
//         return Ok(());
//     }
// }

use std::path::PathBuf;

fn string_from_pathbuf(path: PathBuf) -> String {
  return path.to_string_lossy().as_ref().to_string();
}

#[derive(Debug)]
pub struct CFamilyTarget {
  // id: uuid::Uuid,
  name: String,
  deps: Vec<String>,
  base_dir: PathBuf,
  link_dirs: Vec<PathBuf>,
  include_dirs: Vec<PathBuf>,
  source_files: Vec<PathBuf>,
  compiler: String,
}

// impl Default for CFamilyTarget {
//   fn default() -> Self {
//       return Self {
//         id: uuid::Uuid::new_v4(),
//         deps: vec!(),
//         base_dir: "/".into(),
//         link_dirs: vec!(), include_dirs: vec!(), source_files: vec!(),
//       }
//   }
// }

impl CFamilyTarget {
  fn if_exists<P: AsRef<std::path::Path>>(path: P) -> Result<()> {
    return match std::fs::try_exists(&path) {
        Ok(true) => Ok(()),
        _ => Err(anyhow!("missing file/folder or broken link: {:?}", path.as_ref())),
    }
  }
  fn warn_not_exists<P: AsRef<std::path::Path>>(path: P) {
    Self::if_exists(&path)
      .unwrap_or(println!("WARN: link dir does not exist: {:?}", path.as_ref()));
  }
  pub fn push_link_dir(&mut self, path: PathBuf) -> &mut Self {
    Self::warn_not_exists(&path);
    self.link_dirs.push(path);
    return self;
  }
  pub fn push_include_dir(&mut self, path: PathBuf) -> &mut Self {
    Self::warn_not_exists(&path);
    self.include_dirs.push(path);
    return self;
  }
  pub fn push_link_dirs(&mut self, paths: Vec<PathBuf>) -> &mut Self {
    for path in paths.into_iter() {
      Self::warn_not_exists(&path);
      self.link_dirs.push(path);
    }
    return self;
  }
  pub fn push_include_dirs(&mut self, paths: Vec<PathBuf>) -> &mut Self {
    for path in paths.into_iter() {
      Self::warn_not_exists(&path);
      self.include_dirs.push(path);
    }
    return self;
  }

  pub fn with_base_dir(&mut self, path: PathBuf) -> &mut Self {
    Self::warn_not_exists(&path);
    self.base_dir = path;
    return self;
  }
  pub fn with_source_file(&mut self, file: PathBuf) -> &mut Self {
    Self::warn_not_exists(&file);
    self.source_files.push(file);
    return self;
  }
  pub fn with_source_files(&mut self, files: Vec<PathBuf>) -> &mut Self {
    for file in files {
      Self::warn_not_exists(&file);
      self.source_files.push(file);
    }
    return self;
  }
  pub fn depends_on(&mut self, dep: String) -> &mut Self {
    self.deps.push(dep);
    return self;
  }

  pub fn accumulate(prefix: &str, vector: &Vec<PathBuf>) -> String {
    return vector.clone().into_iter()
      .map(|item| string_from_pathbuf(item))
      .fold(String::new(), |acc, item| {
        acc + prefix + item.as_str()
      });
  }
  
  pub fn from_config(
    config: crate::config::CFamilyConfig,
    compiler: String,
    name: String,
  ) -> Self {
    let sources: Vec<PathBuf> = match config.sources {
      Some(crate::config::SourcesList::Auto) | None => {
        find_sources(config.dir.clone(), "*.c").unwrap()
      },
      Some(crate::config::SourcesList::Files(files)) => { files }
    };
    return Self {
        // id: uuid::Uuid::new_v4(),
        name,
        deps: config.deps.unwrap_or_default(),
        base_dir: config.dir,
        link_dirs: config.links.unwrap_or_default(),
        include_dirs: config.includes.unwrap_or_default(),
        source_files: sources,
        compiler,
    };
  }
}

impl Target for CFamilyTarget {
  fn name(&self) -> String { return self.name.clone(); }
  fn deps(&self) -> Vec<String> { return self.deps.clone(); }
  fn should_recompile(&self) -> bool { todo!(); }
  fn compile(&self) -> Result<()> {
      let links = CFamilyTarget::accumulate(" -L", &self.link_dirs);
      let includes = CFamilyTarget::accumulate(" -I", &self.include_dirs);
      let sources = CFamilyTarget::accumulate(" ", &self.source_files);

      let command_str = self.compiler.clone() + &links + &includes + &sources;

      println!("INFO: Running C Compiler: {}", &command_str);

      let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .current_dir(&self.base_dir)
        .status()?;
      return Ok(());
  }
}

// #[derive(Debug, Default)]
// pub struct GCCTarget( pub CFamilyTarget );

// impl GCCTarget {
//   pub fn default_source_files(&mut self) -> &mut Self {
//     self.0.source_files = find_sources(self.0.base_dir.clone(), "*.c").unwrap();
//     return self;
//   }
// }

// impl Target for GCCTarget {
//     fn dependencies(&self) -> Vec<uuid::Uuid> { return self.0.deps.clone(); }
//     fn uuid(&self) -> uuid::Uuid { return self.0.id; }
//     fn should_recompile(&self) -> bool { todo!(); }
//     fn add_link_dir(&mut self, dir: std::path::PathBuf) { self.0.push_link_dir(dir); }
//     fn add_include_dir(&mut self, dir: std::path::PathBuf) { self.0.push_include_dir(dir); }
//     fn compile(&self) -> Result<()> {
//       let links = CFamilyTarget::accumulate(" -L", &self.0.link_dirs);
//       let includes = CFamilyTarget::accumulate(" -I", &self.0.include_dirs);
//       let sources = CFamilyTarget::accumulate(" ", &self.0.source_files);

//       let command_str = String::from("gcc") + &links + &includes + &sources;

//       println!("INFO: Running C Compiler: {}", &command_str);

//       let _ = std::process::Command::new("sh")
//         .arg("-c")
//         .arg(command_str)
//         .current_dir(&self.0.base_dir)
//         .status()?;
//       return Ok(());
//     }
// }

