use crate::cli::Error;

use crate::irgen::Function;

/// Generate a comment in avrasm
pub fn generate_comment(data: &str) -> Result<String, Error>
{
    Ok(format!("; {}\n", data))
}

/// Generate a command in avrasm
pub fn generate_command(data: &str) -> Result<String, Error>
{
    Ok(format!("     {}\n", data))
}

/// Generate a label in avrasm
pub fn generate_label(data: &str) -> Result<String, Error>
{
    Ok(format!("{}:\n", data))
}

/// Get Label Test for avrasm
pub fn get_label(func: &Function, label: &String) -> Result<String, Error>
{
    Ok(format!("{}{}", func.name, label))
}