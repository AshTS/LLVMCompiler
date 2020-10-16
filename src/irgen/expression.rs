use super::{Function, Value, Literal, Symbol, Instruction, OpCode, attempt_mutate_type, has_unknown_type, get_value_type, correct_type_references, type_from_parse_tree};

use crate::cli::Error;

use super::{expected_got_error, compiler_error_loc};
use super::{DataType, NonPtrType};

use crate::parser::ParseTreeNode;
use crate::parser::ExpressionType as ExpressionTypeP;

use crate::tokenizer::{Token, FileLocation};

use std::cell::RefCell;

/// Expression Types
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType
{
    LogicalAnd,
    LogicalOr,
    FunctionCall,
    LogicalNot,
    ArrayAccess,
    BitwiseNot,
    Ternary,
    UnaryOperation(OpCode, isize),
    DereferenceLeft,
    Cast(DataType),
    Comma,
    UnaryMinus,
    IntegerLiteral,
    Identifier,
    PreExpression(OpCode),
    PostExpression(OpCode),
    BinaryExpression(OpCode),
    AssignmentExpression(Option<OpCode>),
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
                        DataType::new(NonPtrType::Unknown, 0, false)))), vec![], token))
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
            ParseTreeNode::Expression(expr_type, children) =>
            {
                match expr_type
                {
                    // The Unary Plus Does basically Nothing
                    ExpressionTypeP::UnaryPlus => {Expression::from_parse_tree_node(children[0].clone(), func)},
                    ExpressionTypeP::UnaryMinus => 
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::UnaryMinus, None, vec![child0]))
                    },
                    ExpressionTypeP::BitwiseNot => 
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::UnaryMinus, None, vec![child0]))
                    },
                    ExpressionTypeP::LogicalNot => 
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::LogicalNot, None, vec![child0]))
                    },
                    ExpressionTypeP::PreIncrement => 
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::PreExpression(OpCode::Add), None, vec![child0]))
                    },
                    ExpressionTypeP::PreDecrement => 
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::PreExpression(OpCode::Sub), None, vec![child0]))
                    },
                    ExpressionTypeP::PostIncrement => 
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::PostExpression(OpCode::Add), None, vec![child0]))
                    },
                    ExpressionTypeP::PostDecrement => 
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::PostExpression(OpCode::Sub), None, vec![child0]))
                    },
                    ExpressionTypeP::Add | ExpressionTypeP::Subtract | ExpressionTypeP::Multiply | ExpressionTypeP::Divide |
                    ExpressionTypeP::Modulus | ExpressionTypeP::ShiftLeft | ExpressionTypeP::ShiftRight | ExpressionTypeP::LessThan |
                    ExpressionTypeP::LessThanOrEqual | ExpressionTypeP::GreaterThan | ExpressionTypeP::GreaterThanOrEqual |
                    ExpressionTypeP::Equal | ExpressionTypeP::NotEqual | ExpressionTypeP::BitwiseAnd | ExpressionTypeP::BitwiseOr |
                    ExpressionTypeP::BitwiseXor |ExpressionTypeP::ArrayAccess => 
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;
                        let child1 = Expression::from_parse_tree_node(children[1].clone(), func)?;

                        Ok(Expression::new(ExpressionType::BinaryExpression(
                            match expr_type
                            {
                                ExpressionTypeP::Add => OpCode::Add,
                                ExpressionTypeP::Subtract => OpCode::Sub,
                                ExpressionTypeP::Multiply => OpCode::Mul,
                                ExpressionTypeP::Divide => OpCode::Div,
                                ExpressionTypeP::Modulus => OpCode::Mod,
                                ExpressionTypeP::ShiftLeft => OpCode::Shl,
                                ExpressionTypeP::ShiftRight => OpCode::Shr,
                                ExpressionTypeP::LessThan => OpCode::Clt,
                                ExpressionTypeP::LessThanOrEqual => OpCode::Cle,
                                ExpressionTypeP::GreaterThan => OpCode::Cgt,
                                ExpressionTypeP::GreaterThanOrEqual => OpCode::Cge,
                                ExpressionTypeP::Equal => OpCode::Ceq,
                                ExpressionTypeP::NotEqual => OpCode::Cne,
                                ExpressionTypeP::BitwiseAnd => OpCode::And,
                                ExpressionTypeP::BitwiseOr => OpCode::Or,
                                ExpressionTypeP::BitwiseXor => OpCode::Xor,
                                ExpressionTypeP::ArrayAccess => OpCode::Array,
                                _ => {unreachable!();}
                            }
                        ), None, vec![child0, child1]))
                    },
                    ExpressionTypeP::AddAssign =>
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;
                        let child1 = Expression::from_parse_tree_node(children[1].clone(), func)?;

                        Ok(Expression::new(ExpressionType::AssignmentExpression(Some(
                            match expr_type
                            {
                                ExpressionTypeP::AddAssign => OpCode::Add,
                                ExpressionTypeP::SubtractAssign => OpCode::Sub,
                                ExpressionTypeP::MultiplyAssign => OpCode::Mul,
                                ExpressionTypeP::DivideAssign => OpCode::Div,
                                ExpressionTypeP::ModulusAssign => OpCode::Mod,
                                ExpressionTypeP::ShiftLeftAssign => OpCode::Shl,
                                ExpressionTypeP::ShiftRightAssign => OpCode::Shr,
                                ExpressionTypeP::BitwiseAndAssign => OpCode::And,
                                ExpressionTypeP::BitwiseOrAssign => OpCode::Or,
                                ExpressionTypeP::BitwiseXorAssign => OpCode::Xor,
                                _ => {unreachable!();}
                            })
                        ), None, vec![child0, child1]))
                    }
                    ExpressionTypeP::Assignment => 
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;
                        let child1 = Expression::from_parse_tree_node(children[1].clone(), func)?;

                        Ok(Expression::new(ExpressionType::AssignmentExpression(None), None, vec![
                            child0,
                            child1
                        ]))
                    },
                    ExpressionTypeP::Comma =>
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;
                        let child1 = Expression::from_parse_tree_node(children[1].clone(), func)?;

                        Ok(Expression::new(ExpressionType::Comma, None, vec![
                            child0,
                            child1
                        ]))
                    },
                    ExpressionTypeP::Cast =>
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;
                        let datatype = type_from_parse_tree(children[1].clone())?;

                        Ok(Expression::new(ExpressionType::Cast(datatype), None, vec![
                            child0
                        ]))
                    },
                    ExpressionTypeP::Ternary =>
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;
                        let child1 = Expression::from_parse_tree_node(children[1].clone(), func)?;
                        let child2 = Expression::from_parse_tree_node(children[2].clone(), func)?;

                        Ok(Expression::new(ExpressionType::Ternary, None, vec![
                            child0,
                            child1,
                            child2
                        ]))
                    },
                    ExpressionTypeP::Reference =>
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::UnaryOperation(OpCode::Ref, 1), None, vec![
                            child0
                        ]))
                    },
                    ExpressionTypeP::Dereference =>
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::UnaryOperation(OpCode::Deref, -1), None, vec![
                            child0
                        ]))
                    },
                    ExpressionTypeP::DereferenceLeft =>
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;

                        Ok(Expression::new(ExpressionType::DereferenceLeft, None, vec![
                            child0
                        ]))
                    },
                    ExpressionTypeP::FunctionCall =>
                    {
                        let func_name = match &children[0]
                        {
                            ParseTreeNode::Identifier(token) => token.data.clone(),
                            _ => {panic!("")}
                        };

                        let mut new_children = vec![];

                        for child in &children[1..children.len()]
                        {
                            new_children.push(Expression::from_parse_tree_node(child.clone(), func)?);
                        }

                        Ok(Expression::new(ExpressionType::FunctionCall, Some(Value::Label(func_name)), new_children))
                    },
                    ExpressionTypeP::LogicalAnd =>
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;
                        let child1 = Expression::from_parse_tree_node(children[1].clone(), func)?;

                        Ok(Expression::new(ExpressionType::LogicalAnd, None, vec![
                            child0,
                            child1
                        ]))
                    },
                    ExpressionTypeP::LogicalOr =>
                    {
                        let child0 = Expression::from_parse_tree_node(children[0].clone(), func)?;
                        let child1 = Expression::from_parse_tree_node(children[1].clone(), func)?;

                        Ok(Expression::new(ExpressionType::LogicalOr, None, vec![
                            child0,
                            child1
                        ]))
                    },
                    default => {panic!("{:?}", default);}
                }
            },
            default =>
            {
                expected_got_error("an expression", default.clone())
            }
        }
    }

    pub fn render(&mut self, func: &RefCell<&mut Function>) -> Result<(), Error>
    {
        match self.mode.clone()
        {
            ExpressionType::IntegerLiteral | ExpressionType::Identifier=> {},
            ExpressionType::UnaryMinus =>
            {
                self.children[0].render(func)?;
                let val0 = self.children[0].value(func)?;

                let datatype = get_value_type(&val0).unwrap();

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), correct_type_references(datatype.clone())));
                self.value = Some(value.clone());

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Sub, vec![
                    value,
                    Value::Literal(Literal::new(0, datatype)),
                    val0,
                    ]));
            },
            ExpressionType::BitwiseNot =>
            {
                self.children[0].render(func)?;
                let val0 = self.children[0].value(func)?;

                let datatype = get_value_type(&val0).unwrap();

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), correct_type_references(datatype.clone())));
                self.value = Some(value.clone());

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Xor, vec![
                    value,
                    Value::Literal(Literal::new(-1, datatype)),
                    val0,
                    ]));
            },
            ExpressionType::BinaryExpression(opcode) =>
            {
                self.children[0].render(func)?;
                self.children[1].render(func)?;

                let mut val0 = self.children[0].value(func)?;
                let mut val1 = self.children[1].value(func)?;

                let datatype = if !has_unknown_type(&val0) && has_unknown_type(&val1) // First is known
                {
                    get_value_type(&val0).unwrap()
                }
                else if has_unknown_type(&val0) && !has_unknown_type(&val1) // Second is known
                {
                    get_value_type(&val1).unwrap()
                }
                else if !has_unknown_type(&val0) && !has_unknown_type(&val1) // Both Known
                {
                    if get_value_type(&val0).unwrap() == get_value_type(&val1).unwrap()
                    {
                        get_value_type(&val0).unwrap()
                    }
                    else
                    {
                        DataType::new(NonPtrType::Unknown, 0, false)
                    }
                }
                else
                {
                    DataType::new(NonPtrType::Unknown, 0, false)
                };

                val0 = attempt_mutate_type(val0, datatype.clone());
                val1 = attempt_mutate_type(val1, datatype.clone());
                
                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), correct_type_references(datatype)));
                self.value = Some(value.clone());

                func.borrow_mut().add_instruction(Instruction::new(opcode, vec![
                    value,
                    val0,
                    val1,
                    ]));
            },
            ExpressionType::AssignmentExpression(operation) =>
            {
                self.children[0].render(func)?;
                self.children[1].render(func)?;

                let mut val0 = self.children[0].value(func)?;
                let mut val1 = self.children[1].value(func)?;

                let datatype = if !has_unknown_type(&val0) && has_unknown_type(&val1) // First is known
                {
                    get_value_type(&val0).unwrap()
                }
                else if has_unknown_type(&val0) && !has_unknown_type(&val1) // Second is known
                {
                    get_value_type(&val1).unwrap()
                }
                else if !has_unknown_type(&val0) && !has_unknown_type(&val1) // Both Known
                {
                    if get_value_type(&val0).unwrap() == get_value_type(&val1).unwrap()
                    {
                        get_value_type(&val0).unwrap()
                    }
                    else
                    {
                        DataType::new(NonPtrType::Unknown, 0, false)
                    }
                }
                else
                {
                    DataType::new(NonPtrType::Unknown, 0, false)
                };

                val0 = attempt_mutate_type(val0, datatype.clone());
                val1 = attempt_mutate_type(val1, datatype.clone());

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), correct_type_references(datatype)));
                self.value = Some(value.clone());

                match operation
                {
                    Some(opcode) =>
                    {
                        func.borrow_mut().add_instruction(Instruction::new(opcode, vec![
                            val0.clone(),
                            val0.clone(),
                            val1,
                            ]));
                        
                        func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                            value.clone(),
                            val0.clone()
                            ]));
                    },
                    None =>
                    {
                        func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                            val0,
                            val1.clone()
                            ]));
                        func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                            value.clone(),
                            val1.clone()
                            ]));
                        self.value = Some(value.clone());
                    }
                }

                
            },
            ExpressionType::PreExpression(opcode) =>
            {
                self.children[0].render(func)?;

                let val0 = self.children[0].value(func)?;

                func.borrow_mut().add_instruction(Instruction::new(opcode, vec![
                    val0.clone(),
                    val0.clone(),
                    Value::Literal(Literal::new(1, get_value_type(&val0).unwrap())),
                    ]));

                self.value = Some(val0)
            },
            ExpressionType::PostExpression(opcode) =>
            {
                self.children[0].render(func)?;

                let val0 = self.children[0].value(func)?;

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), correct_type_references(get_value_type(&val0).unwrap())));

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                    value.clone(),
                    val0.clone(),
                    ]));

                func.borrow_mut().add_instruction(Instruction::new(opcode, vec![
                    val0.clone(),
                    val0.clone(),
                    Value::Literal(Literal::new(1, get_value_type(&val0).unwrap())),
                    ]));

                self.value = Some(value)
            },
            ExpressionType::Comma =>
            {
                self.children[0].render(func)?;
                self.children[1].render(func)?;

                self.value = Some(self.children[1].value(func)?);
            },
            ExpressionType::Cast(datatype) =>
            {
                self.children[0].render(func)?;
                let mut val0 = self.children[0].value(func)?;

                let corrected_type = correct_type_references(datatype.clone());

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), corrected_type.clone()));

                val0 = attempt_mutate_type(val0, corrected_type);

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Cast, vec![
                    value.clone(),
                    val0.clone()
                    ]));

                self.value = Some(value);
            },
            ExpressionType::Ternary =>
            {
                let body = func.borrow_mut().get_label();
                let clause = func.borrow_mut().get_label();
                let exit = func.borrow_mut().get_label();

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), DataType::new(NonPtrType::Unknown, 0, false)));
                self.value = Some(value.clone());

                self.children[0].render(func)?;

                // Perform the comparison
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Bne, vec![
                    self.children[0].value(func)?, 
                    Value::Literal(Literal::new(0, DataType::new(NonPtrType::Unknown, 0, false))),
                    Value::Label(body.clone()),
                    Value::Label(clause.clone())]));

                // Place the body label
                func.borrow_mut().place_label_here(body.clone());

                self.children[1].render(func)?;

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                    value.clone(),
                    self.children[1].value(func)?,
                    ]));

                // Add a jump statement to skip the clause
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(exit.clone())]));

                // Place the clause label
                func.borrow_mut().place_label_here(clause.clone());

                self.children[2].render(func)?;

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                    value,
                    self.children[2].value(func)?,
                    ]));

                // Place the exit label
                func.borrow_mut().place_label_here(exit.clone());
            },
            ExpressionType::LogicalNot =>
            {
                let body = func.borrow_mut().get_label();
                let clause = func.borrow_mut().get_label();
                let exit = func.borrow_mut().get_label();

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), DataType::new(NonPtrType::Unknown, 0, false)));
                self.value = Some(value.clone());

                self.children[0].render(func)?;

                // Perform the comparison
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Bne, vec![
                    self.children[0].value(func)?, 
                    Value::Literal(Literal::new(0, DataType::new(NonPtrType::Unknown, 0, false))),
                    Value::Label(body.clone()),
                    Value::Label(clause.clone())]));

                // Place the body label
                func.borrow_mut().place_label_here(body.clone());

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                    value.clone(),
                    Value::Literal(Literal::new(0, DataType::new(NonPtrType::Unknown, 0, false))),
                    ]));

                // Add a jump statement to skip the clause
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(exit.clone())]));

                // Place the clause label
                func.borrow_mut().place_label_here(clause.clone());

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                    value,
                    Value::Literal(Literal::new(1, DataType::new(NonPtrType::Unknown, 0, false))),
                    ]));

                // Place the exit label
                func.borrow_mut().place_label_here(exit.clone());
            },
            ExpressionType::UnaryOperation(opcode, delta) =>
            {
                self.children[0].render(func)?;

                let val0 = self.children[0].value(func)?;

                let mut datatype = get_value_type(&val0).unwrap();
                datatype.num_ptr = (datatype.num_ptr as isize + delta) as usize;

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), datatype.clone()));

                func.borrow_mut().add_instruction(Instruction::new(opcode, vec![
                    value.clone(),
                    val0.clone()
                    ]));

                self.value = Some(value);
            },
            ExpressionType::DereferenceLeft =>
            {
                self.children[0].render(func)?;
                let mut val0 = self.children[0].value(func)?;

                let mut datatype = get_value_type(&val0).unwrap().clone();
                datatype.is_ref = true;
                datatype.num_ptr -= 1;

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), datatype.clone()));

                val0 = attempt_mutate_type(val0, datatype);

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Cast, vec![
                    value.clone(),
                    val0.clone()
                    ]));

                self.value = Some(value);
            },
            ExpressionType::FunctionCall =>
            {
                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), DataType::new(NonPtrType::Unknown, 0, false)));

                let l = self.children.len();
                for arg in &mut self.children[0..l]
                {
                    arg.render(func)?;

                    // Push an argument
                    func.borrow_mut().add_instruction(Instruction::new(OpCode::Push, vec![
                        arg.value(func)?]));
                }

                // Call the function
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Call, vec![
                    value.clone(),
                    self.value.clone().unwrap()]));

                self.value = Some(value.clone());

            },
            ExpressionType::LogicalAnd =>
            {
                let body = func.borrow_mut().get_label();
                let clause = func.borrow_mut().get_label();
                let exit = func.borrow_mut().get_label();

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), DataType::new(NonPtrType::Unknown, 0, false)));
                self.value = Some(value.clone());

                self.children[0].render(func)?;

                // Perform the comparison
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Bne, vec![
                    self.children[0].value(func)?, 
                    Value::Literal(Literal::new(0, DataType::new(NonPtrType::Unknown, 0, false))),
                    Value::Label(body.clone()),
                    Value::Label(clause.clone())]));

                // Place the body label
                func.borrow_mut().place_label_here(body.clone());

                self.children[1].render(func)?;

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                    value.clone(),
                    self.children[1].value(func)?,
                    ]));

                // Add a jump statement to skip the clause
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(exit.clone())]));

                // Place the clause label
                func.borrow_mut().place_label_here(clause.clone());

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                    value,
                    Value::Literal(Literal::new(0, DataType::new(NonPtrType::Unknown, 0, false))),
                    ]));

                // Place the exit label
                func.borrow_mut().place_label_here(exit.clone());
            },
            ExpressionType::LogicalOr =>
            {
                let body = func.borrow_mut().get_label();
                let clause = func.borrow_mut().get_label();
                let exit = func.borrow_mut().get_label();

                let value = Value::Symbol(Symbol::new(func.borrow_mut().get_register(), DataType::new(NonPtrType::Unknown, 0, false)));
                self.value = Some(value.clone());

                self.children[0].render(func)?;

                // Perform the comparison
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Bne, vec![
                    self.children[0].value(func)?, 
                    Value::Literal(Literal::new(0, DataType::new(NonPtrType::Unknown, 0, false))),
                    Value::Label(body.clone()),
                    Value::Label(clause.clone())]));

                // Place the body label
                func.borrow_mut().place_label_here(body.clone());

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                    value.clone(),
                    Value::Literal(Literal::new(1, DataType::new(NonPtrType::Unknown, 0, false))),
                    ]));

                // Add a jump statement to skip the clause
                func.borrow_mut().add_instruction(Instruction::new(OpCode::Jmp, vec![Value::Label(exit.clone())]));

                // Place the clause label
                func.borrow_mut().place_label_here(clause.clone());

                self.children[1].render(func)?;

                func.borrow_mut().add_instruction(Instruction::new(OpCode::Mov, vec![
                    value.clone(),
                    self.children[1].value(func)?,
                    ]));

                // Place the exit label
                func.borrow_mut().place_label_here(exit.clone());
            },
            _ => {unimplemented!()}
        }

        Ok(())
    }

    pub fn value(&self, _func: &RefCell<&mut Function>) -> Result<Value, Error>
    {
        Ok(self.value.clone().unwrap())
    }
}