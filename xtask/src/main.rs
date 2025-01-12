use anda::cli::Cli;
use anyhow::Result;
use clap::{Command, CommandFactory};
use clap_complete::{generate_to, shells::Shell};
use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

fn main() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("manpage") => manpage()?,
        Some("completion") => completion()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
manpage            builds application and man pages
completion         builds shell completions
"
    )
}

/// WARN: Consumes subcommands
fn gen_manpage(cmd: Rc<Command>, man_dir: &Path) {
    let name = cmd.get_display_name().map(|s| s.to_string()).unwrap_or_else(|| cmd.clone().get_name().to_string());
    if name.starts_with("anda-help") {
        return;
    }
    let mut out = File::create(man_dir.join(format!("{name}.1"))).unwrap();
    {
        // HACK 'static
        let name: &'static str = Box::leak(Box::new(name));
        let man_cmd = (*cmd).clone().name(name);
        clap_mangen::Man::new(man_cmd).render(&mut out).unwrap();
    }
    out.flush().unwrap();

    for sub in (*cmd).clone().get_subcommands_mut() {
        // let sub = sub.clone().display_name("anda-b");
        gen_manpage(Rc::new(std::mem::take(sub)), man_dir)
    }
}

fn manpage() -> Result<()> {
    let app = Rc::new({
        let mut cmd = Cli::command();
        cmd.build();
        cmd
    });
    let out_dir = "target";
    let man_dir = PathBuf::from(&out_dir).join("man_pages");

    create_dir_all(&man_dir).unwrap();

    gen_manpage(app.clone(), &man_dir);

    let path = PathBuf::from(&out_dir).join("assets");

    let man_dir = path.join("man_pages");
    std::fs::create_dir_all(&man_dir).unwrap();
    gen_manpage(app, &man_dir);

    Ok(())
}

fn completion() -> Result<()> {
    let mut app = Cli::command();
    app.build();

    let out_dir = "target";
    let completion_dir = PathBuf::from(&out_dir).join("assets/completion");

    let shells: Vec<(Shell, &str)> = vec![
        (Shell::Bash, "bash"),
        (Shell::Fish, "fish"),
        (Shell::Zsh, "zsh"),
        (Shell::PowerShell, "pwsh"),
        (Shell::Elvish, "elvish"),
    ];

    for (shell, name) in shells {
        let dir = completion_dir.join(name);
        std::fs::create_dir_all(&dir).unwrap();
        generate_to(shell, &mut app, "anda", dir)?;
    }

    Ok(())
}
