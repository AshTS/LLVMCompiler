use std::fmt;

use crate::llvm::DataType;
use crate::llvm::NonPtrType;
use crate::llvm::expected_got_error;

use crate::parser::ParseTreeNode;

use crate::cli::Error;

use super::{type_from_parse_tree, identifier_from_parse_tree, arguments_from_parse_tree};

#[derive(Debug, Clone)]
pub struct Function
{
    name: String,
    return_type: DataType,
    arguments: Vec<(String, DataType)>
}

impl Function
{
    pub fn new(name: String, return_type: DataType, arguments: Vec<(String, DataType)>) -> Self
    {
        Self
        {
            name,
            return_type,
            arguments
        }
    }

    pub fn from_parse_tree_node(node: ParseTreeNode) -> Result<Self, Error>
    {
        match node
        {
            ParseTreeNode::Function(children) =>
            {
                Ok(Function::new(identifier_from_parse_tree(children[1].clone())?,
                                 type_from_parse_tree(children[0].clone())?,
                                 arguments_from_parse_tree(children[2].clone())?))
            },
            default =>
            {
                expected_got_error("Function", default)
            }
        }
    }
}

impl fmt::Display for Function
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // Return values
        write!(f, "define {} @{}(", self.return_type, self.name)?;

        // Arguments
        for (i, (t, name)) in self.arguments.iter().enumerate()
        {
            write!(f, "{} %{}", name, t)?; 
            if i != self.arguments.len() - 1
            {
                write!(f, ", ");
            }
        }

        writeln!(f, ")")?;

        writeln!(f, "{{")?;

        writeln!(f, "}}")?;

        Ok(())
    }
}