use crate::cli::Error;
use crate::tokenizer::{Token, FileLocation};

pub fn parse_error<T>(location: FileLocation, text: String) -> Result<T, Error>
{
    Err(Error::error(&format!("Parse Error: {} at {}", text, location)))
}

pub fn expected_got_error<T>(expected: &str, got: &Token) -> Result<T, Error>
{
    parse_error(got.clone().location, format!("Expected {}, got '{}'", expected, got.data))
}

pub fn unexpected_eof_error<T>(expected: &str, last: &Token) -> Result<T, Error>
{
    let mut loc = last.clone().location;
    loc.col += last.data.len();
    parse_error(loc, format!("Unexpected EOF while parsing, expected {}", expected))
}