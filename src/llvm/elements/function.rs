use std::fmt;

use crate::llvm::{NonPtrType, DataType};
use crate::llvm::expected_got_error;

use crate::parser::ParseTreeNode;

use crate::cli::Error;

use super::{type_from_parse_tree, identifier_from_parse_tree, arguments_from_parse_tree};
use super::{Statement, StatementContext};

use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Function
{
    name: String,
    return_type: DataType,
    arguments: Vec<(String, DataType)>,
    statement: Statement
}

impl Function
{
    pub fn new(name: String, return_type: DataType, arguments: Vec<(String, DataType)>, statement: Statement) -> Self
    {
        Self
        {
            name,
            return_type,
            arguments,
            statement
        }
    }

    pub fn from_parse_tree_node(node: ParseTreeNode) -> Result<Self, Error>
    {
        match node
        {
            ParseTreeNode::Function(children) =>
            {
                let name = identifier_from_parse_tree(children[1].clone())?;
                let return_type = type_from_parse_tree(children[0].clone())?;
                let arguments = arguments_from_parse_tree(children[2].clone())?;

                let context = RefCell::new(StatementContext::new(return_type.clone()));

                let statement = Statement::from_parse_tree_node(children[3].clone(), &context)?;

                Ok(Function::new(name, return_type, arguments, statement))
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
                write!(f, ", ")?;
            }
        }

        writeln!(f, ")")?;

        write!(f, "{{\n{}", self.statement)?;

        if self.return_type.raw_type == NonPtrType::Void && self.return_type.num_ptr == 0
        {
            writeln!(f, "ret void")?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}