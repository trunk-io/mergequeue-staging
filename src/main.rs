use clap::Parser;
use confique::Config;
use gen::cli::{Cli, Subcommands};
use gen::config::Conf;
use gen::edit::change_file;
use std::path::PathBuf;
use walkdir::WalkDir;

use std::time::Instant;

fn get_txt_files() -> std::io::Result<Vec<PathBuf>> {
    let mut path = std::env::current_dir()?;
    path.push("bazel/");
    let mut paths = Vec::new();
    for entry in WalkDir::new(&path) {
        let entry = entry?;
        if entry.file_type().is_file()
            && entry.path().extension().and_then(std::ffi::OsStr::to_str) == Some("txt")
        {
            paths.push(entry.path().to_path_buf());
        }
    }
    Ok(paths)
}

fn run() -> anyhow::Result<()> {
    let start = Instant::now();
    let cli: Cli = Cli::parse();

    if let Some(Subcommands::Genconfig {}) = &cli.subcommand {
        Conf::print_default();
        return Ok(());
    }

    let config = Conf::builder()
        .env()
        .file("demo.toml")
        .file(".config/demo.toml")
        .load()
        .unwrap_or_else(|err| {
            eprintln!("Generator cannot run: {}", err);
            std::process::exit(1);
        });

    println!("{:?}", config.mode);

    println!("{}", cli.gh_token);

    let files = get_txt_files()?;
    let filenames: Vec<String> = files
        .into_iter()
        .map(|path| path.to_string_lossy().into_owned())
        .collect();
    change_file(&filenames)?;

    Ok(())
}

fn main() {
    env_logger::init();

    match run() {
        Ok(_) => (),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
