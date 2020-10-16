use crate::cli::Error;
use crate::parser::ParseTreeNode;
use crate::tokenizer::FileLocation;

/// Display a compiler error (from IR code gen)
pub fn compiler_error<T>(text: String) -> Result<T, Error>
{
    Err(Error::error(&format!("Compilation Error: {}", text)))
}

/// Expected, got style error for IR code gen
pub fn expected_got_error<T>(expected: &str, got: ParseTreeNode) -> Result<T, Error>
{
    let raw_got_str = format!("{:?}", got);
    let got_str = raw_got_str.split("(").nth(0).unwrap();

    compiler_error(format!("Expected {}, got {}", expected, got_str))
}

/// Display the location if the location is known
pub fn compiler_error_loc<T>(text: String, loc: &Option<FileLocation>) -> Result<T, Error>
{
    match loc
    {
        None => Err(Error::error(&format!("Compilation Error: {}", text))),
        Some(l) => Err(Error::error(&format!("Compilation Error: {}", format!("{} at {}", text, l))))
    }
    
}