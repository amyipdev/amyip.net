use std::ffi::OsString;

mod common;
mod sysvars;
mod vfs;

use clap::{Parser, Subcommand};

const DESC: &str = const_format::formatcp!(
    "\x1b[39;49;1minfsprogs\x1b[0m {} - {}\nCopyright (c) 2024 {}",
    clap::crate_version!(),
    clap::crate_description!(),
    clap::crate_authors!()
);

#[derive(Parser)]
#[command(name = "infsprogs")]
#[command(author, version)]
#[command(about = DESC)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Unpack { target: OsString },
    Build { dir: OsString },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Unpack { target } => unpack(target),
        Commands::Build { dir } => build(dir),
    }
}

fn unpack(target: OsString) {}
fn build(target: OsString) {}
