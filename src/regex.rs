//! Regex pattern parsing and password generation.

use std::{ops::Range, str::Chars};

use crate::error::ParseError;

/// Maximum length for a regex unit repetition.
pub const UNIT_MAXLEN: usize = 128;

/// All printable ASCII characters (space to DEL).
const PRINTABLE_CHARS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

/// A single unit in a regex pattern, matching a range of repetitions
/// of a set of characters.
#[derive(Debug, Clone)]
pub struct RegexUnit {
    chars: Vec<char>,
    range: Range<usize>,
}

impl RegexUnit {
    /// Creates an empty unit that matches nothing.
    pub fn empty() -> Self {
        Self {
            chars: Vec::new(),
            range: 1..1,
        }
    }

    /// Creates a unit that matches any printable ASCII character.
    pub fn full() -> Self {
        Self {
            chars: PRINTABLE_CHARS.chars().collect(),
            range: 1..1,
        }
    }

    /// Returns true if this unit has no characters.
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
}

/// Parses a character class [...] inside a regex pattern.
fn parse_char_class(chars: &mut Chars<'_>) -> Result<Vec<char>, ParseError> {
    let mut pool = Vec::new();
    let negate = chars.clone().peekable().next().map_or(false, |c| c == '^');
    if negate {
        chars.next();
    }

    loop {
        match chars.next() {
            Some(']') => {
                if negate {
                    let pool_set: Vec<char> = pool.clone();
                    let excluded: Vec<char> = PRINTABLE_CHARS
                        .chars()
                        .filter(|c| !pool_set.contains(c))
                        .collect();
                    break Ok(excluded);
                } else {
                    break Ok(pool);
                }
            }
            Some('\\') => match chars.next() {
                Some(c) if "-]\\".contains(c) => pool.push(c),
                Some(c) => {
                    return Err(ParseError::WrongToken {
                        at: 0,
                        got: c,
                        expected: "\\\\ \\- or \\]".into(),
                    });
                }
                None => return Err(ParseError::UnexpectedEOF),
            },
            Some(c) => {
                if chars.clone().peekable().next().map_or(false, |n| n == '-') {
                    chars.next();
                    match chars.next() {
                        Some(c2) => pool.extend(c..=c2),
                        None => return Err(ParseError::UnexpectedEOF),
                    }
                } else {
                    pool.push(c);
                }
            }
            None => return Err(ParseError::UnexpectedEOF),
        }
    }
}

/// Parses a repetition range {min,max}.
fn parse_repetition_range(chars: &mut Chars<'_>) -> Result<Range<usize>, ParseError> {
    let mut from = 0;
    let mut first = true;
    let mut buffer = String::new();

    loop {
        match chars.next() {
            Some('}') => {
                let to = buffer.parse::<usize>().unwrap_or(0).min(UNIT_MAXLEN);
                if first {
                    break Ok(to..to);
                } else {
                    break Ok(from..(to + 1));
                }
            }
            Some(',') => {
                if !first {
                    break Err(ParseError::WrongToken {
                        at: 0,
                        got: ',',
                        expected: "digit or '}'".into(),
                    });
                }
                first = false;
                from = buffer.parse().unwrap_or(0);
                buffer.clear();
            }
            Some(c) if c.is_ascii_digit() => buffer.push(c),
            Some(c) => {
                break Err(ParseError::WrongToken {
                    expected: "digit ',' or '}'".into(),
                    at: 0,
                    got: c,
                });
            }
            None => break Err(ParseError::UnexpectedEOF),
        }
    }
}

/// Parses an escape sequence and returns the matching character set.
fn parse_escape_sequence(chars: &mut Chars<'_>) -> Result<Vec<char>, ParseError> {
    match chars.next() {
        Some(special) if "[]{}.*?+\\".contains(special) => Ok(vec![special]),
        Some('d') => Ok("0123456789".chars().collect()),
        Some('w') => Ok(
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .collect(),
        ),
        Some(c) => Err(ParseError::WrongToken {
            at: 0,
            got: c,
            expected: "[]{}.*?+\\ or classes \\w \\d".into(),
        }),
        None => Err(ParseError::UnexpectedEOF),
    }
}

/// Compiles a regex pattern string into a vector of RegexUnits.
pub fn compile(pattern: &str) -> Result<Vec<RegexUnit>, ParseError> {
    let mut chars = pattern.chars();
    let mut units = Vec::new();
    let mut current_unit = RegexUnit::empty();

    while let Some(c) = chars.next() {
        match c {
            '[' => {
                if !current_unit.is_empty() {
                    units.push(current_unit.clone());
                }
                current_unit = RegexUnit {
                    chars: parse_char_class(&mut chars)?,
                    range: 1..1,
                };
            }
            '.' => {
                if !current_unit.is_empty() {
                    units.push(current_unit.clone());
                }
                current_unit = RegexUnit::full();
            }
            '\\' => {
                if !current_unit.is_empty() {
                    units.push(current_unit.clone());
                }
                current_unit = RegexUnit {
                    chars: parse_escape_sequence(&mut chars)?,
                    range: 1..1,
                };
            }
            '{' => current_unit.range = parse_repetition_range(&mut chars)?,
            '*' => current_unit.range = 0..UNIT_MAXLEN,
            '?' => current_unit.range = 0..1,
            '+' => current_unit.range = 1..UNIT_MAXLEN,
            _ => {
                if !current_unit.is_empty() {
                    units.push(current_unit.clone());
                }
                current_unit = RegexUnit {
                    chars: vec![c],
                    range: 1..1,
                };
            }
        }
    }
    if !current_unit.is_empty() {
        units.push(current_unit);
    }

    Ok(units)
}

/// Selects a random character from a slice using modulo.
fn select_random_char(chars: &[char], rand_val: u128) -> char {
    chars[(rand_val as usize) % chars.len()]
}

/// Generates a string by repeatedly selecting random characters
/// from each unit's character set.
pub fn generate_string<I: Iterator<Item = u128>>(regex: Vec<RegexUnit>, mut rand: I) -> String {
    let mut buffer = String::with_capacity(UNIT_MAXLEN * 16);
    for RegexUnit { chars, range } in regex {
        let min = range.start;
        let max = range.end;
        let len = if min == max {
            min
        } else {
            (rand.next().unwrap() as usize) % (max - min) + min
        };
        for _ in 0..len {
            buffer.push(select_random_char(&chars, rand.next().unwrap()));
        }
    }
    buffer.shrink_to_fit();
    buffer
}
