use std::fmt;
use super::FileLocation;

#[derive(Debug, Clone)]
pub struct Token
{
    pub location: FileLocation,
    pub data: String
}

impl Token
{
    pub fn new(location: FileLocation, data: String) -> Self
    {
        Self
        {
            location,
            data
        }
    }
}

impl fmt::Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:?}{}\t{}", self.data, if format!("{:?}", self.data).len() >= 8 {""} else {"\t"}, self.location)
    }
}