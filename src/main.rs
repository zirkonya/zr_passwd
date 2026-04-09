//! Password generator using regex patterns.

use std::error::Error;

use clap::Parser;

use crate::{
    error::ParseError,
    random::Random,
    regex::{compile, generate_string},
};

mod error;
mod random;
mod regex;

/// Command-line arguments.
#[derive(Parser)]
struct Args {
    username: String,
    pattern: String,
    /// Passphrase for password generation (optional, will prompt if not provided).
    #[arg(short, long)]
    passphrase: Option<String>,
}

/// Generates a password from username, passphrase, and regex pattern.
fn generate_password(
    username: &str,
    passphrase: &str,
    pattern: &str,
) -> Result<String, ParseError> {
    let units = compile(pattern)?;
    let random = Random::from_inputs(username, passphrase);
    Ok(generate_string(units, random.iter().into_iter()))
}

/// Prompts the user for a passphrase using secure input.
fn ask_passphrase() -> std::io::Result<String> {
    rpassword::prompt_password("enter the passphrase : ")
}

/// Main entry point.
fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        username,
        pattern,
        passphrase,
    } = Args::parse();

    let passphrase = match passphrase {
        Some(p) => p,
        None => ask_passphrase()?,
    };

    let password = generate_password(&username, &passphrase, &pattern)?;
    println!("{password}");
    Ok(())
}
