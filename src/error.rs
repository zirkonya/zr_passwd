//! Error types for regex parsing.

use std::{
    error::Error,
    fmt::{self, Display},
    sync::Arc,
};

/// Errors that can occur during regex pattern parsing.
#[derive(Clone)]
pub enum ParseError {
    /// Unexpected end of input.
    UnexpectedEOF,
    /// Encountered an unexpected token.
    WrongToken {
        /// Position in the input string where the error occurred.
        at: usize,
        /// The character that was found.
        got: char,
        /// Description of what was expected.
        expected: Arc<str>,
    },
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedEOF => write!(f, "UnexpectedEOF"),
            ParseError::WrongToken { at, got, expected } => {
                if f.alternate() {
                    writeln!(f, "WrongToken {{")?;
                    writeln!(f, "    at: {},", at)?;
                    writeln!(f, "    got: '{}',", got)?;
                    writeln!(f, "    expected: {},", expected)?;
                    write!(f, "}}")
                } else {
                    write!(
                        f,
                        "WrongToken {{ at: {}, got: '{}', expected: \"{}\" }}",
                        at, got, expected
                    )
                }
            }
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}
