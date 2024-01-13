
#![feature(fs_try_exists)]

#[macro_use] extern crate anyhow;
#[macro_use] extern crate serde;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod build;
// use build::*;

mod target;
mod config;

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
    /// choose how to run lowbuild
    #[command(subcommand)]
    action: Command,
}


fn main() -> Result<()> {

    use ron::extensions::Extensions;

    let args = Args::parse();

    let ron_parser = ron::Options::default()
        .with_default_extension(Extensions::IMPLICIT_SOME)
        .with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES);
        // unavailable for some reason
        // .with_default_extension(Extensions::EXPLICIT_STRUCT_NAMES);

    std::fs::try_exists("./lowbuild.ron")
        .expect("now config file lowbuild.ron in this directory");
    let buildconfig: config::BuildConfig = ron_parser.from_str(
        &std::fs::read_to_string("lowbuild.ron")?
    )?;

    println!("config: {:?}", buildconfig);

    match &args.action {
        Command::Run { name } => {
            todo!();
            // let build = Build::new()
        },
        Command::Build{ name } => {
            let mut builder = build::Build::new_file(buildconfig, name.clone())?;
            println!("builder: {:?}", builder);
            builder.build()?;
        }
        Command::BuildAll => {
            build::Build::new(buildconfig)?.build()?;
        },
    }

    return Ok(());
}
