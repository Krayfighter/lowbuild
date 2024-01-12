
#![feature(fs_try_exists)]

#[macro_use] extern crate anyhow;
#[macro_use] extern crate serde;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod build;
use build::*;

mod target;

#[derive(Subcommand)]
enum Command {
    BuildAll,
    Build {
        name: String,
    },
    Run {
        name: String,
    }
}

#[derive(Parser)]
#[command(
    author = "Aiden J Kring",
    version = "0.1.0",
    about = "build system",
    long_about = None,
)]
struct Args {
    // choose how to run lowbuild
    #[command(subcommand)]
    action: Command,
    // #[commands(subcommands)]
    // commands: Option<Commands>,
}


fn main() -> Result<()> {

    let args = Args::parse();
    
    std::fs::try_exists("./lowbuild.toml")?;
    let config_str = std::fs::read_to_string("lowbuild.toml")?;
    let config_toml = config_str.as_str().parse::<toml::Table>()?;
    // println!("raw file:\n{}", &config_str);
    // println!("file parsed:\n{}", config_toml);

    match &args.action {
        Command::Run { name } => {
            todo!();
            // let build = Build::new()
        },
        Command::Build{ name } => {
            let build = Build::new_file(config_toml, name.clone())?;
            build.build()?;
        }
        Command::BuildAll => {
            Build::new(config_toml)?.build()?;
        },
        // None => {
        //     println!("must use a command {run, build}");
        // }
    }

    return Ok(());
}
