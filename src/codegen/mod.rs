use crate::cli::{Error};

use crate::irgen::Function;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodegenMode
{
    IntermediateRepresentation,
    AvrAssembly,
    Unknown
}

impl CodegenMode
{
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

#[derive(Debug, Clone)]
pub struct CodeGenerator
{
    mode: CodegenMode,
    functions: Vec<Function>
}

impl CodeGenerator
{
    pub fn new(mode: CodegenMode, functions: Vec<Function>) -> Self
    {
        Self
        {
            mode,
            functions
        }
    }

    pub fn render(&self) -> Result<String, Error>
    {
        let mut result = String::new();

        match self.mode
        {
            CodegenMode::Unknown => {return Err(Error::fatal_error("Unknown Codegen Mode"));},
            CodegenMode::IntermediateRepresentation =>
            {
                for func in &self.functions
                {
                    result += &format!("{}\n", func);
                }
            },
            CodegenMode::AvrAssembly =>
            {
                unimplemented!();
            }
        }

        Ok(result)
    }
}