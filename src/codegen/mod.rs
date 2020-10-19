mod avrasm;
mod llvm;

use crate::cli::{Error, Options};

use crate::irgen::Function;

/// Code Generation Mode
/// What language the output will be in
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodegenMode
{
    IntermediateRepresentation,
    AvrAssembly,
    LLVM,
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
            "llvm" => CodegenMode::LLVM,
            _ => CodegenMode::Unknown
        }
    }
}

/// Wrapper which compiles the individually translated functions
#[derive(Debug, Clone)]
pub struct CodeGenerator
{
    mode: CodegenMode,
    functions: Vec<Function>,
    options: Options
}

impl CodeGenerator
{
    /// Generate a new CodeGenerator object
    pub fn new(mode: CodegenMode, functions: Vec<Function>, options: Options) -> Self
    {
        Self
        {
            mode,
            functions,
            options
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
            },
            CodegenMode::LLVM =>
            {
                // Extract the target triple if passed
                let target = if let Some(args) = self.options.map.get("--llvm-target")
                {
                    Some(args[0].as_str())
                }
                else
                {
                    None
                };

                // Extract the target data layout if passed
                let layout = if let Some(args) = self.options.map.get("--llvm-layout")
                {
                    Some(args[0].as_str())
                }
                else
                {
                    None
                };

                // Invoke the renderer for the LLVM code generaor
                result = format!("{}", llvm::LLVMGenerator::new(self.functions.clone()).render(target, layout)?)
            }
        }

        Ok(result)
    }
}