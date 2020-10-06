use super::{Function, Value, Literal};

use crate::cli::Error;

use crate::llvm::{expected_got_error, compiler_error_loc};
use crate::llvm::{DataType, NonPtrType};

use crate::parser::ParseTreeNode;

use crate::tokenizer::{Token, FileLocation};

use std::cell::RefCell;

/// Expression Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExpressionType
{
    ArrayAccess,
    FunctionCall,
    PostIncrement,
    PostDecrement,
    PreIncrement,
    PreDecrement,
    UnaryPlus,
    UnaryMinus,
    LogicalNot,
    BitwiseNot,
    Dereference,
    Reference,
    Multiply,
    Divide,
    Modulus,
    Add,
    Subtract,
    ShiftLeft,
    ShiftRight,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    Ternary,
    Assignment,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModulusAssign,
    ShiftLeftAssign,
    ShiftRightAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,
    Cast,
    Comma,
    IntegerLiteral,
    Identifier
}

#[derive(Debug, Clone)]
pub struct Expression
{
    mode: ExpressionType,
    value: Option<Value>,
    children: Vec<Expression>,
    pos: Option<FileLocation>
}

impl Expression
{
    pub fn new(mode: ExpressionType, value: Option<Value>, children: Vec<Expression>) -> Self
    {
        Self
        {
            mode,
            value,
            children,
            pos: None
        }
    }

    pub fn new_with_token(mode: ExpressionType, value: Option<Value>, children: Vec<Expression>, token: &Token) -> Self
    {
        Self
        {
            mode,
            value,
            children: children,
            pos: Some(token.location.clone())
        }
    }


    pub fn from_parse_tree_node(node: ParseTreeNode, func: &RefCell<&mut Function>) -> Result<Self, Error>
    {
        match &node
        {
            ParseTreeNode::IntegerLiteral(token) =>
            {
                Ok(Expression::new_with_token(ExpressionType::IntegerLiteral, 
                    Some(Value::Literal(Literal::new(i128::from_str_radix(token.data.as_str(), 10).unwrap(),
                        DataType::new(NonPtrType::I32, 0)))), vec![], token))
            },
            ParseTreeNode::Identifier(token) =>
            {
                let val = Value::Symbol(match func.borrow_mut().symbol_table.get(&token.data)
                {
                    Some(v) => v.clone(),
                    None => {compiler_error_loc(format!("Symbol {} not found in symbol table", token.data), &Some(token.location.clone()))?;unreachable!()}
                });

                Ok(Expression::new(ExpressionType::Identifier,
                    Some(val), vec![]))
            },
            default =>
            {
                expected_got_error("an expression", default.clone())
            }
        }
    }

    pub fn render(&self, _func: &RefCell<&mut Function>) -> Result<(), Error>
    {
        match self.mode
        {
            ExpressionType::IntegerLiteral | ExpressionType::Identifier =>{},
            _ => {unimplemented!()}
        }

        Ok(())
    }

    pub fn value(&self, _func: &RefCell<&mut Function>) -> Result<Value, Error>
    {
        match self.mode
        {
            ExpressionType::IntegerLiteral | ExpressionType::Identifier => {Ok(self.value.clone().unwrap())},
            _ => {unimplemented!()}
        }
    }
}