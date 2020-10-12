use crate::llvm::{NonPtrType, DataType};
use super::Value;

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