use std::fmt;

use crate::llvm::expected_got_error;

use crate::parser::ParseTreeNode;

use crate::cli::Error;


#[derive(Debug, Clone)]
pub enum ExpressionType
{
    IntegerLiteral(i128),
    Identifier(String)
}


#[derive(Debug, Clone)]
pub struct Expression
{
    expr_type: ExpressionType
}

impl Expression
{
    pub fn new(expr_type: ExpressionType) -> Self
    {
        Self
        {
            expr_type
        }
    }

    pub fn from_parse_tree_node(node: ParseTreeNode) -> Result<Self, Error>
    {
        match &node
        {
            ParseTreeNode::IntegerLiteral(token) =>
            {
                Ok(Expression::new(ExpressionType::IntegerLiteral(i128::from_str_radix(token.data.as_str(), 10).unwrap())))
            },
            ParseTreeNode::Identifier(token) =>
            {
                Ok(Expression::new(ExpressionType::Identifier(token.data.clone())))
            },
            default =>
            {
                expected_got_error("an expression", default.clone())
            }
        }
    }

    pub fn get_symbol(&self) -> String
    {
        match &self.expr_type
        {
            ExpressionType::IntegerLiteral(val) =>
            {
                format!("{}", val)
            },
            ExpressionType::Identifier(val) =>
            {
                format!("%{}", val)
            }
        }
    }
}

impl fmt::Display for Expression
{
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self.expr_type
        {
            ExpressionType::IntegerLiteral(_) | ExpressionType::Identifier(_) => {} // Integer Literal has no setup
        }
        Ok(())
    }
}