use crate::cli::Error;

use crate::irgen::Function;
use crate::irgen::{DataType, NonPtrType};

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

/// Get the size of a datatype
pub fn get_size_datatype(t: DataType) -> usize
{
    if t.num_ptr == 0
    {
        match t.raw_type
        {
            NonPtrType::I8 | NonPtrType::U8 => 1,
            NonPtrType::I16 | NonPtrType::U16 => 2,
            _ => unimplemented!()
        }
    }
    else
    {
        2
    }
}