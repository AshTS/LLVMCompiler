use std::fs::File;
use std::io::prelude::*;

use super::cli;

/// Input File Struct
pub struct InputFile
{
    pub data: String,
    pub filename: String
}

impl InputFile
{
    /// Generate a new input file
    pub fn new(filename: String) -> Result<Self, cli::Error>
    {
        Ok(Self
        {
            data: read_from_file(filename.clone())?,
            filename: filename
        })
    }
}

/// Get a string from a file
fn raw_read_from_file(filename: String) -> std::io::Result<String>
{
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Read from a filename with a Cli Result
pub fn read_from_file(filename: String) -> Result<String, cli::Error>
{
    match raw_read_from_file(filename.clone())
    {
        Ok(v) => Ok(v),
        Err(_) => Err(cli::Error::error(&format!("{}: No such file or directory", filename.clone())))
    }
}