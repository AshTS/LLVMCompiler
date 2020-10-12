use super::{Function, Value, Literal, Expression, Instruction, OpCode, Symbol, attempt_mutate_type};

use crate::cli::Error;

use crate::llvm::{expected_got_error, compiler_error, compiler_error_loc};
use crate::llvm::{DataType, NonPtrType, type_from_parse_tree, identifier_from_parse_tree};

use crate::parser::ParseTreeNode;

use crate::tokenizer::{Token, FileLocation};

use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum StatementType
{
    Empty,
    CompoundStatement,
    ContinueStatement,
    BreakStatement,
    InitializationStatement,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    LoopStatement,
    ReturnStatement,
    ExpressionStatement
}

#[derive(Debug, Clone)]
pub struct Statement
{
    mode: StatementType,
    pub expr: Option<Expression>,
    children: Vec<Statement>,
    pos: Option<FileLocation>,
    init_data: Option<(DataType, String)>
}

impl Statement
{
    pub fn new(mode: StatementType) -> Self
    {
        Self
        {
            mode,
            expr: None,
            children: vec![],
            pos: None,
            init_data: None
        }
    }

    pub fn new_with_token(mode: StatementType, token: &Token) -> Self
    {
        Self
        {
            mode,
            expr: None,
            children: vec![],
            pos: Some(token.location.clone()),
            init_data: None
        }
    }

    pub fn add_child(&mut self, child: Self)
    {
        self.children.push(child);
    }

    pub fn from_parse_tree_node(node: ParseTreeNode, func: &RefCell<&mut Function>) -> Result<Self, Error>
    {
        match &node
        {
            ParseTreeNode::Empty =>
            {
                Ok(Statement::new(StatementType::Empty))
            },
            ParseTreeNode::Statement(children) =>
            {
                // If the length of the children is zero, it is a noop
                if children.len() == 0
                {
                    Ok(Statement::new(StatementType::Empty))
                }
                else
                {
                    match &children[0]
                    {
                        ParseTreeNode::RawToken(token) =>
                        {
                            if token.data == "continue"
                            {
                                Ok(Statement::new_with_token(StatementType::ContinueStatement, &token))
                            }
                            else if token.data == "break"
                            {
                                Ok(Statement::new_with_token(StatementType::BreakStatement, &token))
                            }
                            else
                            {
                                compiler_error(format!("Expected 'continue' or 'break', got '{}'", token.data))?;
                                unreachable!();
                            }
                        },
                        ParseTreeNode::Expression(_, _) =>
                        {
                            let mut result = Statement::new(StatementType::ExpressionStatement);

                            result.expr = Some(Expression::from_parse_tree_node(children[0].clone(), func)?);

                            Ok(result)
                        }
                        default =>
                        {
                            expected_got_error("RawToken or Expression", default.clone())?;
                            unreachable!();
                        }
                    }
                }
            },
            // Compound Statement
            ParseTreeNode::Statements(children) =>
            {
                let mut result = Statement::new(StatementType::CompoundStatement);

                for child in children
                {
                    result.add_child(Statement::from_parse_tree_node(child.clone(), func)?)
                }

                Ok(result)
            },
            // Return Statement
            ParseTreeNode::ReturnStatement(children) =>
            {
                let mut result = Statement::new(StatementType::ReturnStatement);

                result.expr = Some(Expression::from_parse_tree_node(children[0].clone(), func)?);

                Ok(result)
            },
            // Loop Statement
            ParseTreeNode::Loop(children) =>
            {
                let mut result = Statement::new(StatementType::LoopStatement);

                result.add_child(Statement::from_parse_tree_node(children[0].clone(), func)?);

                Ok(result)
            },
            // If Statement
            ParseTreeNode::IfStatement(children) =>
            {
                let mut result = Statement::new(StatementType::IfStatement);

                // Condition
                result.expr = Some(Expression::from_parse_tree_node(children[0].clone(), func)?);

                // Body
                result.add_child(Statement::from_parse_tree_node(children[1].clone(), func)?);
                
                // Clause
                result.add_child(Statement::from_parse_tree_node(children[2].clone(), func)?);

                Ok(result)
            },
            // While Statement
            ParseTreeNode::WhileLoop(children) =>
            {
                let mut result = Statement::new(StatementType::WhileStatement);

                // Condition
                result.expr = Some(Expression::from_parse_tree_node(children[0].clone(), func)?);

                // Body
                result.add_child(Statement::from_parse_tree_node(children[1].clone(), func)?);

                Ok(result)
            },
            // Do While Statement
            ParseTreeNode::DoWhileLoop(children) =>
            {
                let mut result = Statement::new(StatementType::DoWhileStatement);

                // Condition
                result.expr = Some(Expression::from_parse_tree_node(children[0].clone(), func)?);

                // Body
                result.add_child(Statement::from_parse_tree_node(children[1].clone(), func)?);
                
                Ok(result)
            },
            // Initialization
            ParseTreeNode::AssignmentStatement(children) =>
            {
                let mut result = Statement::new(StatementType::CompoundStatement);

                let datatype = type_from_parse_tree(children[0].clone())?;

                match &children[1]
                {
                    ParseTreeNode::Assignments(assignment_children) =>
                    {
                        for child in assignment_children
                        {
                            match child
                            {
                                ParseTreeNode::Assignment(assignment_data) =>
                                {
                                    let mut temp = Statement::new(StatementType::InitializationStatement);

                                    let s = identifier_from_parse_tree(assignment_data[0].clone())?;

                                    temp.init_data = Some((datatype, s.clone()));
                                    temp.expr = Some(Expression::from_parse_tree_node(assignment_data[1].clone(), func)?);

                                    func.borrow_mut().symbol_table.insert(s.clone(), Symbol::new(s.clone(), datatype.clone()));

                                    result.add_child(temp);
                                },
                                default =>
                                {
                                    expected_got_error("an assignment", default.clone())?;
                                    unreachable!();
                                }
                            }
                        }
                    },
                    default =>
                    {
                        expected_got_error("assignments", default.clone())?;
                        unreachable!();
                    }
                }

                Ok(result)
            }
            default =>
            {
                expected_got_error("a statement", default.clone())
            }
        }
    }

    pub fn render(&self, func: &RefCell<&mut Function>) -> Result<(), Error>
    {
        match self.mode
        {
            StatementType::Empty =>
            {
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Nop, vec![]))
            },
            StatementType::CompoundStatement =>
            {
                // Loop over all children and render those statements
                for child in &self.children
                {
                    child.render(func.clone())?;
                }
            },
            StatementType::ContinueStatement =>
            {
                let mut f = func.borrow_mut();

                match f.get_continue()
                {
                    Some(v) =>
                    {
                        f.add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(v)]))
                    },
                    None => 
                    {
                        compiler_error_loc(format!("Cannot use continue statement outside of loop"), &self.pos)?
                    }
                }
            },
            StatementType::BreakStatement =>
            {
                let mut f = func.borrow_mut();

                match f.get_break()
                {
                    Some(v) =>
                    {
                        f.add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(v)]))
                    },
                    None => 
                    {
                        compiler_error_loc(format!("Cannot use break statement outside of loop"), &self.pos)?
                    }
                }
            },
            StatementType::InitializationStatement =>
            {
                let mut e = self.expr.clone().unwrap();
                let symbol = func.borrow_mut().symbol_table.get(&self.init_data.clone().unwrap().1).unwrap().clone();

                // Render the expression
                e.render(func.clone())?;

                let value = attempt_mutate_type(e.value(func)?, symbol.datatype);

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Alloc, vec![
                    Value::Symbol(symbol),
                    value
                    ]));
            },
            StatementType::IfStatement =>
            {
                let body = func.borrow_mut().get_label();
                let clause = func.borrow_mut().get_label();
                let exit = func.borrow_mut().get_label();

                let mut e = self.expr.clone().unwrap();

                // Render the expression
                e.render(func.clone())?;

                // Perform the comparison
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Bne, vec![
                    e.value(func)?, 
                    Value::Literal(Literal::new(0, DataType::new(NonPtrType::Unknown, 0, false))),
                    Value::Label(body.clone()),
                    Value::Label(clause.clone())]));
                
                // Place the body label
                func.borrow_mut().place_label_here(body.clone());

                // Render the body
                self.children[0].render(func)?;

                // Add a jump statement to skip the clause
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(exit.clone())]));

                // Place the clause label
                func.borrow_mut().place_label_here(clause.clone());

                // Render the clause
                self.children[0].render(func)?;

                // Place the exit label
                func.borrow_mut().place_label_here(exit.clone());
            },
            StatementType::WhileStatement =>
            {
                let (start, end) = func.borrow_mut().enter_loop();
                let allow = func.borrow_mut().get_label();

                let mut e = self.expr.clone().unwrap();

                // Add a label to the start of the loop
                func.borrow_mut().place_label_here(start.clone());

                // Render the expression
                e.render(func.clone())?;

                // Perform the comparison
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Bne, vec![
                    e.value(func)?, 
                    Value::Literal(Literal::new(0, DataType::new(NonPtrType::Unknown, 0, false))),
                    Value::Label(allow.clone()),
                    Value::Label(end.clone())]));
                func.borrow_mut().place_label_here(allow.clone());

                // Render the statement within the loop
                self.children[0].render(func)?;
                
                // Add a jump statement to loop back to the top
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(start)]));

                // Add a label to the end of the loop
                func.borrow_mut().place_label_here(end);

                func.borrow_mut().exit_loop();
            },
            StatementType::DoWhileStatement =>
            {
                let (start, end) = func.borrow_mut().enter_loop();

                let mut e = self.expr.clone().unwrap();

                // Add a label to the start of the loop
                func.borrow_mut().place_label_here(start.clone());

                // Render the statement within the loop
                self.children[0].render(func)?;

                // Render the expression
                e.render(func.clone())?;

                // Perform the comparison
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Bne, vec![
                    e.value(func)?, 
                    Value::Literal(Literal::new(0, DataType::new(NonPtrType::Unknown, 0, false))),
                    Value::Label(start.clone()),
                    Value::Label(end.clone())]));
            
                // Add a label to the end of the loop
                func.borrow_mut().place_label_here(end);

                func.borrow_mut().exit_loop();
            },
            StatementType::LoopStatement =>
            {
                let (start, end) = func.borrow_mut().enter_loop();

                // Add a label to the start of the loop
                func.borrow_mut().place_label_here(start.clone());

                // Render the statement within the loop
                self.children[0].render(func)?;

                // Add a jump statement to loop back to the top
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(start)]));

                // Add a label to the end of the loop
                func.borrow_mut().place_label_here(end);

                func.borrow_mut().exit_loop();
            },
            StatementType::ReturnStatement =>
            {
                let mut e = self.expr.clone().unwrap();

                // Render the expression
                e.render(func.clone())?;

                // Then add the return statement
                let val = e.value(func)?;
                let ret_val = func.borrow().return_value.clone();
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![ret_val, val]));

                // Then jump to the exit
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(String::from("exit"))]))

            },
            StatementType::ExpressionStatement =>
            {
                // Just render the expression
                self.expr.clone().unwrap().render(func.clone())?;
            }
        }

        Ok(())
    }
}