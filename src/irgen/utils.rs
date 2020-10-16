use super::{NonPtrType, DataType};
use super::Value;

use super::{compiler_error, expected_got_error};

use crate::parser::ParseTreeNode;

use crate::cli::Error;

pub fn attempt_mutate_type(value: Value, new_type: DataType) -> Value
{
    match value
    {
        Value::Literal(literal) =>
        {
            let mut lit = literal.clone();

            if lit.datatype.raw_type == NonPtrType::Unknown
            {
                lit.datatype = correct_type_references(new_type);
            }

            Value::Literal(lit)
        },
        _ => value
    }
}

pub fn get_value_type(value: &Value) -> Option<DataType>
{
    match value
    {
        Value::Literal(literal) => Some(literal.datatype),
        Value::Symbol(symbol) => Some(symbol.datatype),
        Value::Label(_) => None
    }
}

pub fn has_unknown_type(value: &Value) -> bool
{
    match get_value_type(value)
    {
        Some(v) => v.raw_type == NonPtrType::Unknown,
        None => true
    }
}

pub fn correct_type_references(datatype: DataType) -> DataType
{
    let mut result = datatype.clone();

    result.is_ref = false;

    result
}

pub fn type_from_parse_tree(node: ParseTreeNode) -> Result<DataType, Error>
{
    match node
    {
        ParseTreeNode::Type(children) =>
        {
            let non_ptr = match &children[0]
            {
                ParseTreeNode::RawType(token) =>
                {
                    match token.data.as_str()
                    {
                        "i8" => NonPtrType::I8,
                        "u8" => NonPtrType::U8,
                        "i16" => NonPtrType::I16,
                        "u16" => NonPtrType::U16,
                        "i32" => NonPtrType::I32,
                        "u32" => NonPtrType::U32,
                        "i64" => NonPtrType::I64,
                        "u64" => NonPtrType::U64,
                        "void" => NonPtrType::Void,

                        default => 
                        {
                            compiler_error(format!("Bad type, '{}'", default))?;
                            unreachable!();
                        }
                    }
                },
                default =>
                {
                    expected_got_error("Type", default.clone())?;
                    unreachable!();
                }
            };

            Ok(DataType::new(non_ptr, children.len() - 1, false))
        },
        default =>
        {
            expected_got_error("Type", default)
        }
    }
}

pub fn identifier_from_parse_tree(node: ParseTreeNode) -> Result<String, Error>
{
    match node
    {
        ParseTreeNode::Identifier(token) =>
        {
            Ok(String::from(token.data))
        },
        default =>
        {
            expected_got_error("Identifier", default)
        }
    }
}

pub fn arguments_from_parse_tree(node: ParseTreeNode) -> Result<Vec<(String, DataType)>, Error>
{
    match node
    {
        ParseTreeNode::Empty =>
        {
            Ok(vec![])
        },
        ParseTreeNode::Arguments(children) =>
        {
            let mut result = vec![];

            for child in children
            {
                match &child
                {
                    ParseTreeNode::Argument(arg_vals) =>
                    {
                        result.push((identifier_from_parse_tree(arg_vals[1].clone())?,
                                     type_from_parse_tree(arg_vals[0].clone())?))
                    }
                    default =>
                    {
                        expected_got_error("Argument", default.clone())?;
                    }
                }
            }
            
            Ok(result)
        }
        default =>
        {
            expected_got_error("Arguments or Empty", default)
        }
    }
}