use std::fmt;

use crate::llvm::DataType;
use crate::llvm::{compiler_error, expected_got_error};

use crate::parser::ParseTreeNode;

use super::Expression;

use crate::cli::Error;

use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum StatementTypes
{
    Empty,
    CompoundStatement(Vec<Statement>),
    ContinueStatement,
    BreakStatement,
    InitializationStatement,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    LoopStatement,
    ReturnStatement(Expression),
    ExpressionStatement
}

#[derive(Debug, Clone)]
pub struct StatementContext
{
    return_type: DataType,
    next_label: usize,
    loop_entries: Vec<usize>,
    loop_exits: Vec<usize>
}

impl StatementContext
{
    pub fn new(return_type: DataType) -> Self
    {
        Self
        {
            return_type,
            next_label: 0,
            loop_entries: vec![],
            loop_exits: vec![]
        }
    }

    pub fn get_label(&mut self) -> usize
    {
        self.next_label += 1;
        self.next_label - 1
    }

    pub fn push_loop(&mut self, start: usize, exit: usize)
    {
        self.loop_entries.push(start);
        self.loop_exits.push(exit);
    }

    pub fn pop_loop(&mut self)
    {
        self.loop_entries.pop();
        self.loop_exits.pop();
    }
}

#[derive(Debug, Clone)]
pub struct Statement
{
    statement: StatementTypes,
    context: RefCell<StatementContext>
}

impl Statement
{
    pub fn new(statement: StatementTypes, context: RefCell<StatementContext>) -> Self
    {
        Self
        {
            statement,
            context: context
        }
    }

    pub fn from_parse_tree_node(node: ParseTreeNode, context: &RefCell<StatementContext>) -> Result<Self, Error>
    {
        Ok(
            match node
            {
                ParseTreeNode::Statement(children) =>
                {
                    // If the length of the children is zero, it is a noop
                    if children.len() == 0
                    {
                        Statement::new(StatementTypes::Empty, context.clone())
                    }
                    else
                    {
                        match &children[0]
                        {
                            ParseTreeNode::RawToken(token) =>
                            {
                                if token.data == "continue"
                                {
                                    Statement::new(StatementTypes::ContinueStatement, context.clone())
                                }
                                else if token.data == "break"
                                {
                                    Statement::new(StatementTypes::BreakStatement, context.clone())
                                }
                                else
                                {
                                    compiler_error(format!("Expected 'continue' or 'break', got '{}'", token.data))?;
                                    unreachable!();
                                }
                            },
                            default =>
                            {
                                expected_got_error("RawToken", default.clone())?;
                                unreachable!();
                            }
                        }
                    }
                },
                ParseTreeNode::Statements(children) =>
                {
                    let mut statements = vec![];

                    for child in children
                    {
                        statements.push(Statement::from_parse_tree_node(child, &context)?);
                    }

                    Statement::new(StatementTypes::CompoundStatement(statements), context.clone())
                },
                ParseTreeNode::ReturnStatement(children) =>
                {
                    Statement::new(StatementTypes::ReturnStatement(Expression::from_parse_tree_node(children[0].clone())?), context.clone())
                }
                default => 
                {
                    expected_got_error("A Statement", default)?;
                    unreachable!();
                }
            }
        )
    }
}

impl fmt::Display for Statement
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match &self.statement
        {
            StatementTypes::Empty => {}, // NoOp Instruction don't print anything
            StatementTypes::CompoundStatement(statements) =>
            {
                for statement in statements
                {
                    writeln!(f, "{}", statement)?;
                }
            },
            StatementTypes::ContinueStatement =>
            {
                unimplemented!();
            },
            StatementTypes::BreakStatement =>
            {
                unimplemented!();
            },
            StatementTypes::InitializationStatement =>
            {
                unimplemented!();
            },
            StatementTypes::IfStatement =>
            {
                unimplemented!();
            },
            StatementTypes::WhileStatement =>
            {
                unimplemented!();
            },
            StatementTypes::DoWhileStatement =>
            {
                unimplemented!();
            },
            StatementTypes::LoopStatement =>
            {
                unimplemented!();
            },
            StatementTypes::ReturnStatement(expr) =>
            {
                write!(f, "{}", expr)?;

                let symbol = expr.get_symbol();

                writeln!(f, "ret {} {}", self.context.borrow().return_type, symbol)?;
            },
            StatementTypes::ExpressionStatement =>
            {
                unimplemented!();
            }
        }

        Ok(())
    }
}