#![deny(rust_2018_idioms)]
mod artifacts;
mod builder;
mod cli;
mod flatpak;
mod oci;
mod rpm_spec;

use anyhow::Result;

use clap::{AppSettings, Args, CommandFactory, Parser, Subcommand};
use cli::{Cli, Command};

fn main() -> Result<()> {
    //println!("Hello, world!");
    let cli = Cli::parse();

    let mut app = Cli::command();

    app.build();

    let sub = app.get_subcommands();

    // for s in sub {
    //     println!("{:?}", s);
    // }

    // let app = Command::command().find_subcommand("build").unwrap().clone();
    // clap_mangen::Man::new(app).render(&mut std::io::stdout()).unwrap();
    // println!("{:?}", &cli);

    match cli.command.clone() {
        Command::Build {
            all,
            project,
            package,
            rpm_opts,
            flatpak_opts,
            oci_opts,
        } => {
            if project.is_none() && !all {
                // print help
                let mut app = Cli::command();
                let mut a = app
                    .find_subcommand_mut("build")
                    .unwrap()
                    .clone()
                    .display_name("anda-build")
                    .name("anda-build");
                a.print_help().unwrap();
                // print help for build subcommand
                return Err(anyhow::anyhow!(
                    "No project specified, and --all not specified."
                ));
            }

            eprintln!("{:?}", &all);
            builder::builder(
                &cli,
                rpm_opts,
                all,
                project,
                package,
                flatpak_opts,
                oci_opts,
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
