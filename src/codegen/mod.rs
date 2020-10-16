mod avrasm;

use crate::cli::{Error};

use crate::irgen::Function;

/// Code Generation Mode
/// What language the output will be in
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodegenMode
{
    IntermediateRepresentation,
    AvrAssembly,
    Unknown
}

impl CodegenMode
{
    /// Generate a new CodegenMode object from a mode passed as an argument
    pub fn from_mode(mode: &str) -> CodegenMode
    {
        match mode
        {
            "ir" => CodegenMode::IntermediateRepresentation,
            "avrasm" => CodegenMode::AvrAssembly,
            _ => CodegenMode::Unknown
        }
    }
}

/// Wrapper which compiles the individually translated functions
#[derive(Debug, Clone)]
pub struct CodeGenerator
{
    mode: CodegenMode,
    functions: Vec<Function>
}

impl CodeGenerator
{
    /// Generate a new CodeGenerator object
    pub fn new(mode: CodegenMode, functions: Vec<Function>) -> Self
    {
        Self
        {
            mode,
            functions
        }
    }

    /// Generate code for the given functions
    pub fn render(&self) -> Result<String, Error>
    {
        let mut result = String::new();

        match self.mode
        {
            CodegenMode::Unknown => {return Err(Error::fatal_error("Unknown Codegen Mode"));},
            CodegenMode::IntermediateRepresentation =>
            {
                // Render each function of intermediate representation
                for func in &self.functions
                {
                    result += &format!("{}\n", func);
                }
            },
            CodegenMode::AvrAssembly =>
            {
                // Invoke the renderer for the AvrAsm code generator
                result = format!("{}", avrasm::AvrAsmGenerator::new(self.functions.clone()).render()?)
            }
        }

        Ok(result)
    }
}