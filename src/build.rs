

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
    // targets: HashMap<uuid::Uuid, (Rc<dyn crate::target::Target>, String)>,
    targets: HashMap<String, Rc<dyn crate::target::Target>>,
}

impl Build {
  fn new_target(
    target_cfg: crate::config::TargetConfig,
    compilers: Option<crate::config::CompilerConfig>,
    name: &str,
  ) -> Result<Rc<dyn Target>> {
    return match target_cfg {
        crate::config::TargetConfig::RustCargo(cfg) => {
          Ok( Rc::from(crate::target::cargo::CargoTarget::from_config(
            cfg, name.to_string()
          )) )
        },
        crate::config::TargetConfig::CLowBuild(cfg) => {
          let compiler: String = match cfg.compiler.clone() {
            crate::config::CompilerOption::Custom(string) => string,
            crate::config::CompilerOption::CC => match compilers {
              Some(compilers) => match compilers.cc {
                Some(cc) => cc,
                None => bail!("Expected CC to be set in `config`")
              },
              None => bail!("Expected CC to be set in `config`"),
            },
            crate::config::CompilerOption::CXX => match compilers {
              Some(compilers) => match compilers.cxx {
                Some(cxx) => cxx,
                None => bail!("Expected CXX to be set in `config`")
              },
              None => bail!("Expected CXX to be set in `config`")
            }
          };
          Ok( Rc::from(crate::target::lbtarget::CFamilyTarget::from_config(
            cfg, compiler, name.to_string()
          )) )
        },
    };
  }

  fn realize_targets(
    cfg: crate::config::BuildConfig
  ) -> Result<HashMap<String, Rc<dyn Target>>> {
    return Ok(cfg.targets.into_iter()
      .map(|(name, target_cfg)| { (
        name.clone(), Self::new_target(
          target_cfg,
          cfg.config.clone(),
          name.as_str()).unwrap()
      ) })
      .collect()
    );
  }

  fn find_deps_recursive(
    targets: HashMap<String, Rc<dyn Target>>,
    target: Rc<dyn Target>
  ) -> Result<()> {
    for dep in target.deps().into_iter() {
      if !targets.keys().find(|key| {
        key.as_str() == dep.as_str()
      }).is_some() { bail!("unable to find {} in targets", dep); }

      Self::find_deps_recursive(
        targets.clone(),
        targets.get(&dep).unwrap().clone()
      )?;
    }

    return Ok(());
  }

  pub fn new(cfg: crate::config::BuildConfig) -> Result<Self> {
    let targets = Self::realize_targets(cfg)?;

    // ensure that all dpendencies are accounted for
    for (_, target) in targets.iter() {
      Self::find_deps_recursive(targets.clone(), target.clone())?;
    }
    return Ok(Self { targets } );
  }

  pub fn new_file(cfg: crate::config::BuildConfig, build_target: String) -> Result<Self> {
    let targets = Self::realize_targets(cfg)?;

    let main_target = targets.get(&build_target).ok_or(
      anyhow!("target {} not found in targets", build_target)
    )?.clone();
    Self::find_deps_recursive(targets.clone(), main_target.clone())?;

    // filter out only the necessary targets so that
    // unnecessary (non-dependency) targets don't
    // get commpiled
    let deps = main_target.deps();
    let targets = targets.into_iter()
      .filter(|(name, _target)| {
        !deps.contains(name)
      })
      .collect::<HashMap<String, Rc<dyn Target>>>();

    return Ok( Self { targets } );
  }

  fn build_target(
    targets: &mut HashMap<String, Rc<dyn Target>>,
    target: Rc<dyn Target>
  ) -> Result<()> {
    let deps = target.deps();
    if deps.len() > 0 { for dep in deps.iter() {
        Self::build_target(targets, targets.get(dep).unwrap().clone())?;
    } }
    target.compile()?;
    targets.remove(&target.name());
    return Ok(());
  }

  pub fn build(&mut self) -> Result<()> {
    // loop through the targets in self
    // and recursively compile dependencies
    // then remove them from self so they
    // don't get compiled twice, and once
    // self.targets is empty, end the loop
    loop {
      // if self.targets.keys().len() == 0 { break; }
      let target = self.targets.iter().next();
      if target.is_none() { break; }

      let target = target.unwrap().1.clone();
      Self::build_target(&mut self.targets, target)?;
    }
    return Ok(());
  }

  pub fn print_dbg(&self) {
    for (_id, target) in self.targets.iter() {
      println!("target: {:?}\n\n{}", target, target.name())
    }
  }
}
