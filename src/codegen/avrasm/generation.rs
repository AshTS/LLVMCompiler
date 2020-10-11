use crate::cli::Error;
use crate::irgen::{Function};

use super::FunctionGenerationContext;

#[derive(Debug, Clone)]
pub struct AvrAsmGenerator
{
    functions: Vec<Function>
}

impl AvrAsmGenerator
{
    pub fn new(functions: Vec<Function>) -> Self
    {
        Self
        {
            functions
        }
    }

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