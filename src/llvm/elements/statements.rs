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
    IfStatement(Expression, Vec<Statement>),
    WhileStatement,
    DoWhileStatement,
    LoopStatement(Vec<Statement>),
    ReturnStatement(Expression),
    ExpressionStatement(Expression)
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
            next_label: 1,
            loop_entries: vec![],
            loop_exits: vec![]
        }
    }

    pub fn get_label(&mut self) -> usize
    {
        self.next_label += 1;
        self.next_label - 1
    }

    pub fn push_loop(&mut self) -> (usize, usize)
    {
        let start = self.get_label();
        let exit = self.get_label();

        self.loop_entries.push(start);
        self.loop_exits.push(exit);

        (start, exit)
    }

    pub fn pop_loop(&mut self)
    {
        self.loop_entries.pop();
        self.loop_exits.pop();
    }

    pub fn get_continue(&self) -> usize
    {
        self.loop_entries[self.loop_entries.len() - 1]
    }

    pub fn get_break(&self) -> usize
    {
        self.loop_exits[self.loop_exits.len() - 1]
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
                            ParseTreeNode::Expression(_, _) =>
                            {
                                Statement::new(StatementTypes::ExpressionStatement(Expression::from_parse_tree_node(children[0].clone(), &context.clone())?), context.clone())
                            }
                            default =>
                            {
                                expected_got_error("RawToken or Expression", default.clone())?;
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
                ParseTreeNode::IfStatement(children) =>
                {
                    let expr = Expression::from_parse_tree_node(children[0].clone(), &context.clone())?;
                    let statements = vec![Statement::from_parse_tree_node(children[1].clone(), context)?,
                                          Statement::from_parse_tree_node(children[2].clone(), context)?];
                    Statement::new(StatementTypes::IfStatement(expr, statements), context.clone())
                },
                ParseTreeNode::Loop(children) =>
                {
                    let statements = vec![Statement::from_parse_tree_node(children[0].clone(), context)?];
                    Statement::new(StatementTypes::LoopStatement(statements), context.clone())
                },
                ParseTreeNode::ReturnStatement(children) =>
                {
                    Statement::new(StatementTypes::ReturnStatement(Expression::from_parse_tree_node(children[0].clone(), &context.clone())?), context.clone())
                },
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
                writeln!(f, "br label %a{}", self.context.borrow().get_continue())?;
            },
            StatementTypes::BreakStatement =>
            {
                writeln!(f, "br label %a{}", self.context.borrow().get_break())?;
            },
            StatementTypes::InitializationStatement =>
            {
                unimplemented!();
            },
            StatementTypes::IfStatement(expr, statements) =>
            {
                let temp = format!("%a{}", self.context.borrow_mut().get_label());
                let body_label = self.context.borrow_mut().get_label();
                let else_label = self.context.borrow_mut().get_label();

                write!(f, "{}", expr)?;
                writeln!(f, "{} = icmp ne i32 {}, 0", temp, expr.get_symbol())?;
                writeln!(f, "br i1 {}, label %a{}, label %a{}", temp, body_label, else_label)?;
                writeln!(f, "a{}:", body_label)?;
                write!(f, "{}", statements[0])?;
                writeln!(f, "a{}:", else_label)?;
                write!(f, "{}", statements[1])?;
            },
            StatementTypes::WhileStatement =>
            {
                unimplemented!();
            },
            StatementTypes::DoWhileStatement =>
            {
                unimplemented!();
            },
            StatementTypes::LoopStatement(children) =>
            {
                let (continue_label, break_label) = self.context.borrow_mut().push_loop();

                writeln!(f, "a{}:", continue_label)?;

                write!(f, "{}", children[0])?;
                writeln!(f, "br label %a{}", continue_label)?;

                writeln!(f, "a{}:", break_label)?;

                self.context.borrow_mut().pop_loop();
            },
            StatementTypes::ReturnStatement(expr) =>
            {
                write!(f, "{}", expr)?;

                let symbol = expr.get_symbol();

                writeln!(f, "ret {} {}", self.context.borrow().return_type, symbol)?;
            },
            StatementTypes::ExpressionStatement(expr) =>
            {
                write!(f, "{}", expr)?;
            }
        }

        Ok(())
    }
}