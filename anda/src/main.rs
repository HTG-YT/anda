#![deny(rust_2018_idioms)]
mod artifacts;
mod builder;
mod flatpak;
mod oci;
mod rpm_spec;

use anyhow::{Result};


use clap::{AppSettings, Parser, Subcommand};
use std::{path::PathBuf};

use self::artifacts::PackageType;

#[derive(Parser, Debug)]
#[clap(about, version)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Path to Andaman configuration file
    #[clap(default_value = "anda.hcl", short, long, env = "ANDA_CONFIG")]
    config: PathBuf,

    /// Output directory for built packages
    #[clap(short, long, env = "TARGET_DIR", default_value = "anda-build")]
    target_dir: PathBuf,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// Build a project
    Build {
        /// Builds all projects in the current directory
        #[clap(short, long, action)]
        all: bool,

        /// Project to build
        #[clap()]
        project: Option<String>,

        /// Builds a specific artifact format
        #[clap(short, long, arg_enum, default_value = "all")]
        package: PackageType,

        //TODO: Move this to an argument group (clap 4.0 feature(?))
        /// Mock: Do not mirror repositories.
        /// This is useful for quickly building test repositories
        /// without having to wait for the mirror to finish.
        /// This argument is ignored if the build is not RPM Mock.
        #[clap(long, action)]
        no_mirrors: bool,

        /// RPM: Builder backend
        /// possible values: rpmbuild, mock
        /// default: mock
        #[clap(long, arg_enum, default_value = "mock")]
        rpm_builder: rpm_spec::RPMBuilder,

        /// Mock: Mock configuration
        #[clap(long, short = 'c')]
        mock_config: Option<String>,
    },
    /// Cleans up the build directory
    Clean,
}

fn main() -> Result<()> {
    //println!("Hello, world!");
    let cli = Cli::parse();

    println!("{:?}", &cli);

    match cli.command.clone() {
        Command::Build {
            all,
            project,
            package,
            no_mirrors,
            rpm_builder,
            mock_config,
        } => {
            builder::builder(
                &cli,
                all,
                project,
                package,
                no_mirrors,
                rpm_builder,
                mock_config,
            )?;
        }
        Command::Clean => {
            println!("Cleaning up build directory");
            let clean = std::fs::remove_dir_all(&cli.target_dir);
            if clean.is_err() {
                // match the errors
                match clean.err().unwrap().kind() {
                    std::io::ErrorKind::NotFound => {}
                    e => {
                        println!("Error cleaning up build directory: {:?}", e);
                    }
                }
            }
        }
    }
    Ok(())
}
