use crate::cli::Error;

use crate::irgen::{Function};

/// A wrapper for giving a context to code generation for an LLVM function
pub struct FunctionGenerationContext
{
    func: Function
}

impl FunctionGenerationContext
{
    /// Generate a new function generation context object
    pub fn new(func: Function) -> Self
    {
        Self
        {
            func
        }
    }

    /// Render an IR function in LLVM IR
    pub fn render_function(&mut self) -> Result<String, Error>
    {
        let mut result = String::new();

        Ok(result)
    }
}