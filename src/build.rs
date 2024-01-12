

use std::{collections::HashMap, rc::Rc};

use anyhow::Result;

use crate::target::Target;



#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum CCompiler {
    GCC,
    Clang,
}

impl CCompiler {
    fn parse_string(string: &str) -> Option<Self> {
        return match string {
            "gcc" => Some( Self::GCC ),
            "clang" => Some( Self::Clang ),
            _ => None
        }
    }

    pub fn string(&self) -> String {
      return match self {
        Self::GCC => String::from("gcc"),
        Self::Clang => String::from("clang"),
      }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum CppCompiler {
    Gpp,
    Clangpp,
}

impl CppCompiler {
    fn parse_string(string: &str) -> Option<Self> {
        return match string {
            "gcc" | "g++" | "gpp" => Some( Self::Gpp ),
            "clang++" | "clang" => Some( Self::Clangpp ),
            _ => None
        }
    }

    pub fn string(&self) -> String {
      return match self {
        Self::Gpp => String::from("g++"),
        Self::Clangpp => String::from("Clang++"),
      }
    }
}


#[derive(Default, Debug)]
pub struct Build {
    targets: HashMap<uuid::Uuid, (Rc<dyn crate::target::Target>, String)>,
    cc: Option<CCompiler>,
    cxx: Option<CppCompiler>,
}

impl Build {
  pub fn add_target(&mut self, target: toml::Value, name: String) -> Result<()> {

    let target = match target {
      toml::Value::Table(table) => table.clone(),
      targ => bail!(format!("type of [target] is incorrect: {}", targ)),
    };
    
    let base_dir = match target.get("dir").ok_or(
      anyhow!("required `dir` attribute missing in target")
    )? {
      toml::Value::String(string) => string.clone(),
      _ => bail!("invalid type for `dir`")
    };

    let build_type = target.get("build").ok_or(anyhow!("required `build` attribute missing in target"))?;
    let build: Rc<dyn crate::target::Target> = match build_type {
      toml::Value::String(string) => {
        match string.as_str() {
          // `includes` is currently not supported for cargo builds
          "rust-cargo" => {
            Rc::from(crate::target::cargo::CargoTarget::new(base_dir)?)
          },
          "c-lowbuild" => {
            let mut lb_c_target = crate::target::lbtarget::LBCTarget::new(
              base_dir, self.cc.expect("no C Compiler configured").string()
            )?;
            match target.get("includes") {
              Some(toml::Value::Array( includes )) => {
                for include in includes {
                  match include {
                    toml::Value::String( include_dir ) => {
                      // println!("adding include dir: {}", include_dir);
                      lb_c_target.add_include_dir(include_dir.clone().into());
                    },
                    _ => bail!("invalide type in include: {:?}-\n-{}", includes, include),
                  }
                }
              },
              Some( other ) => bail!("invalid include type: {}", other),
              None => {},
            }
            // println!("include dirs (init): {:?}", lb_c_target.include_dirs);
            Rc::from(lb_c_target)
          }
          _ => bail!(format!("unrecognized build type: {}", string))
        }
      },
      _ => bail!("invalid type for `build`")
    };

    self.targets.insert(build.uuid(), (build, name));
    return Ok(());
  }

  pub fn config(&mut self, table: toml::Table) -> Result<()> {
      #[derive(Deserialize)]
      struct BuildConfig {
          CC: Option<String>,
          CXX: Option<String>,
      }
      let config: BuildConfig = table.try_into()?;

      let cc = match config.CC {
          Some(c) => Some( CCompiler::parse_string(c.as_str()).ok_or(
              anyhow!("invalid C compiler string")
          )? ),
          None => None,
      };

      let cxx = match config.CXX {
          Some(c) => Some( CppCompiler::parse_string(c.as_str()).ok_or(
              anyhow!("invalid C++ compiler string")
          )? ),
          None => None,
      };

      self.cc = cc;
      self.cxx = cxx;
      return Ok(());
  }

  fn parse_config(&mut self, build: &toml::Table) -> Result<()> {
    match build.get("config") {
      Some(toml::Value::Table(cfg)) => self.config(cfg.clone())?,
      None => {},
      Some(other) => bail!("invalid type for `config`: {}", other),
    }
    return Ok(());
  }

  fn parse_targets(
    build: &toml::Table
  ) -> Result<toml::map::Map<String, toml::Value>> {
    return match build.get("target") {
      Some( toml::Value::Table(targets) ) => Ok(targets.clone()),
      Some( other ) => bail!("invalid type in target: [{}]", other),
      None => bail!("No jobs \"target\"s to do"),
    }
  }

  pub fn new(build: toml::Table) -> Result<Self> {
    let mut _self = Self::default();
    
    _self.parse_config(&build).unwrap();

    let targets = Self::parse_targets(&build).unwrap();
    for (name, target) in targets {
        _self.add_target(target, name).unwrap();
    }
    return Ok(_self);
  }

  pub fn new_file(build: toml::Table, name: String) -> Result<Self> {
    let mut _self = Self::default();
    _self.parse_config(&build).unwrap();

    let targets = Self::parse_targets(&build).unwrap();

    let main_target = targets.get(&name).expect(
      &format!("unable to find {} in targets", name));

    // for (tname, target) in targets {
    //   _self.add_target(target, tname).unwrap();
    // }
    _self.add_target(main_target.clone(), name).unwrap();
    return Ok(_self);
  }

  pub fn build(&self) -> Result<()> {
    // // // leaves in the dpendency tree (no dependencies)
    // // let targets = self.targets.iter()
    // //   .map(|(_, (target, _))| target.dependencies())
    // //   .flatten()
    // //   .collect::<Vec<uuid::Uuid>>();
    // let leaves = self.targets.iter()
    //   .map(|(_, (target, _))| target.clone())
    //   .filter(|target| {
    //     target.dependencies().len() != 0
    //     // (*target).dependencies().len() != 0
    //     // return targets.contains(target)
    //   });

    // TODO implement dependency
    // loop {
    //   // 
    // }
    for (_uuid, (target, _name)) in self.targets.iter() {
      if target.dependencies().len() != 0 { bail!("dependency tree not yet implemented") }
      target.compile()?;
    }
    return Ok(());
  }

  pub fn print_dbg(&self) {
    for (_id, (target, name)) in self.targets.iter() {
      println!("target: {:?}\n\n{}", target, name)
    }
  }
}
