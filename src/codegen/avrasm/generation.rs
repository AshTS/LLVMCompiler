use crate::cli::Error;
use crate::irgen::{Function};

use super::FunctionGenerationContext;

/// Wrapper for the AVR Assembly Code Generator
#[derive(Debug, Clone)]
pub struct AvrAsmGenerator
{
    functions: Vec<Function>
}

impl AvrAsmGenerator
{
    /// Generate a new AvrAsmGenerator from a vector of IR functions
    pub fn new(functions: Vec<Function>) -> Self
    {
        Self
        {
            functions
        }
    }

    /// Render each function in turn
    pub fn render(self) -> Result<String, Error>
    {
        let mut result = String::new();

        for function in self.functions
        {
            let mut context = FunctionGenerationContext::new(function);
            result += &format!("{}", context.render_function()?);
        }

        Ok(result)
    }
}