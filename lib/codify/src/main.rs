// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]
#![allow(unused)]

mod commands {
    pub mod convert;
}
use commands::*;

mod exit;

use crate::exit::ExitCode;
use clientele::{
    crates::clap::{Args, Parser, Subcommand},
    StandardOptions,
};
use std::{error::Error, path::PathBuf, str::FromStr};

/// Codify.rs Command-Line Interface (CLI)
#[derive(Debug, Parser)]
#[command(name = "Codify.rs")]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show the current configuration
    Convert {},
}

pub fn main() -> Result<(), ExitCode> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    if options.flags.version {
        println!("codify {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if options.flags.license {
        println!("This is free and unencumbered software released into the public domain.");
        return Ok(());
    }

    match options.command {
        Command::Convert {} => convert::convert(),
    }
}
