// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]
#![allow(unused)]

mod exit;

use crate::exit::ExitCode;
use clientele::{
    crates::clap::{Parser, Subcommand},
    StandardOptions,
};
use codify::*;
use std::str::FromStr;

/// Codify.rs Command-Line Interface (CLI)
#[derive(Debug, Parser)]
#[command(name = "Codify.rs")]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Convert a type from one language to another
    Convert {
        /// The qualified source type (e.g., "cpp:float")
        #[clap(value_parser = parse_type)]
        source: (Language, String),

        /// The target language (e.g., "go")
        #[clap(value_parser = parse_language, default_value = "rust")]
        target: Language,
    },
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

    match options.command.unwrap() {
        Command::Convert { source, target } => convert(source, target),
    }
}

pub fn convert(
    (input_language, input_type): (Language, String),
    output_language: Language,
) -> Result<(), ExitCode> {
    let input_type = input_language
        .parse_type(&input_type)
        .expect("input type syntax should have been validated");

    let output_type = output_language.from_type(
        input_type
            .to_rust()
            .expect("input type should be convertible to a Rust type"),
    )?;

    println!("{}", output_type);

    Ok(())
}

fn parse_language(input: &str) -> Result<Language, TypeParseError> {
    Ok(Language::from_str(input).map_err(|_| TypeParseError::InvalidLanguage)?)
}

fn parse_type(input: &str) -> Result<(Language, String), TypeParseError> {
    let input = input.trim().replace('-', " ");
    let Some((input_lhs, input_rhs)) = input.split_once(':') else {
        return Err(TypeParseError::InvalidSyntax);
    };
    let language = Language::from_str(input_lhs).map_err(|_| TypeParseError::InvalidLanguage)?;
    let r#type = language
        .parse_type(input_rhs)
        .map_err(|_| TypeParseError::InvalidType)?;
    Ok((language, r#type.to_string()))
}

#[derive(Clone, Copy, Debug)]
pub enum TypeParseError {
    InvalidSyntax,
    InvalidLanguage,
    InvalidType,
}

impl core::fmt::Display for TypeParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use TypeParseError::*;
        match self {
            InvalidSyntax => write!(f, "invalid syntax"),
            InvalidLanguage => write!(f, "invalid language name"),
            InvalidType => write!(f, "invalid type name"),
        }
    }
}

impl std::error::Error for TypeParseError {}
