use crate::cli::Error;
use crate::irgen::{Function};

use super::FunctionGenerationContext;

/// Wrapper for the LLVM IR Code Generator
#[derive(Debug, Clone)]
pub struct LLVMGenerator
{
    functions: Vec<Function>
}

impl LLVMGenerator
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
    pub fn render(self, target: Option<&str>, datalayout: Option<&str>) -> Result<String, Error>
    {
        let mut result = String::new();

        if let Some(datalayout_str) = datalayout
        {
            result += &format!("target datalayout = \"{}\"\n", datalayout_str);
        }

        if let Some(target_str) = target
        {
            result += &format!("target triple = \"{}\"\n", target_str);
        }

        for function in self.functions
        {
            let mut context = FunctionGenerationContext::new(function);
            result += &format!("{}", context.render_function()?);
        }

        Ok(result)
    }
}