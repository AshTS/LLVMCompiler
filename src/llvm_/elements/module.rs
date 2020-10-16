use std::fmt;

use crate::cli::Error;

use crate::llvm::TargetTriple;
use crate::llvm::expected_got_error;

use super::Function;

use crate::parser::ParseTreeNode;

#[derive(Debug, Clone)]
pub struct Module
{
    data_layout: String,
    target_triple: TargetTriple,
    functions: Vec<Function>
}

impl Module
{
    pub fn new(target_triple: TargetTriple) -> Self
    {
        Self
        {
            data_layout: String::from("e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v64:64:64-v128:128:128-a0:0:64-s0:64:64-f80:128:128-n8:16:32:64-S128"),
            target_triple,
            functions: vec![]
        }
    }

    pub fn load_from_parse_tree(&mut self, node: ParseTreeNode) -> Result<(), Error>
    {
        match node
        {
            ParseTreeNode::Library(children) =>
            {
                for child in children
                {
                    self.add_function(Function::from_parse_tree_node(child)?);
                }
            },
            default =>
            {
                expected_got_error("Library", default)?;
            }
        }

        Ok(())
    }

    pub fn add_function(&mut self, func: Function)
    {
        self.functions.push(func);
    }
}

impl fmt::Display for Module
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // Target Layout
        writeln!(f, "target datalayout = \"{}\"", self.data_layout)?;
        writeln!(f, "target triple = \"{}\"", self.target_triple)?;

        // Generate the functions
        for func in &self.functions
        {
            writeln!(f, "{}", func)?;
        }

        Ok(())
    }
}