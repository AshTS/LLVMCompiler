use crate::cli::Error;
use crate::parser::ParseTreeNode;

pub fn compiler_error<T>(text: String) -> Result<T, Error>
{
    Err(Error::error(&format!("Compilation Error: {}", text)))
}

pub fn expected_got_error<T>(expected: &str, got: ParseTreeNode) -> Result<T, Error>
{
    let raw_got_str = format!("{:?}", got);
    let got_str = raw_got_str.split("(").nth(0).unwrap();

    compiler_error(format!("Expected {}, got {}", expected, got_str))
}