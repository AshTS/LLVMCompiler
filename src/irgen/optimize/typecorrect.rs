use crate::irgen::{Function, Value, DataType, NonPtrType, OpCode};
use crate::irgen::{force_mutate_type};

use std::collections::HashMap;

/// Correct the types within the instructions in an IR Function
pub fn correct_types(f: Function) -> Function
{
    let mut func = f.clone();

    let mut symbol_map: HashMap<String, DataType> = HashMap::new();

    loop
    {
        let mut changed = false;

        for i in 0..func.instructions.len()
        {
            if let Some(inst) = func.instructions.get_mut(&i)
            {
                if inst.opcode == OpCode::Array {continue;}

                let mut datatype = DataType::new(NonPtrType::Unknown, 0, false);

                for arg in &inst.arguments
                {
                    if match arg
                    {
                        Value::Label(_) => {false},
                        Value::Literal(lit) => {if datatype.raw_type == NonPtrType::Unknown && lit.datatype.raw_type != NonPtrType::Unknown {datatype = lit.datatype; true} else {false}},
                        Value::Symbol(symb) => {if datatype.raw_type == NonPtrType::Unknown && symb.datatype.raw_type != NonPtrType::Unknown {datatype = symb.datatype; true} else {false}},
                    }
                    {
                        break;
                    }
                }

                for i in 0.. inst.arguments.len()
                {
                    let v = force_mutate_type(inst.arguments[i].clone(), datatype);

                    if v != inst.arguments[i].clone()
                    {
                        changed = true;
                    }

                    if let Value::Symbol(symb) = v.clone()
                    {
                        if !symbol_map.contains_key(&symb.title) && symb.datatype.raw_type != NonPtrType::Unknown
                        {
                            symbol_map.insert(symb.title, symb.datatype);
                        }
                    }

                    inst.arguments[i] = v;
                }
            }
        }

        for i in 0..func.instructions.len()
        {
            if let Some(inst) = func.instructions.get_mut(&i)
            {
                for i in 0.. inst.arguments.len()
                {
                    let v = inst.arguments[i].clone();

                    if let Value::Symbol(mut symb) = v.clone()
                    {
                        if symb.datatype.raw_type == NonPtrType::Unknown && symbol_map.contains_key(&symb.title)
                        {
                            symb.datatype = symbol_map.get(&symb.title).unwrap().clone();
                            inst.arguments[i] = Value::Symbol(symb);
                            changed = true;
                        }
                    }
                }
            }
        }

        if !changed
        {
            break;
        }
    }

    // Once all changes have been made, default to i32
    for i in 0..func.instructions.len()
    {
        if let Some(inst) = func.instructions.get_mut(&i)
        {
            for i in 0.. inst.arguments.len()
            {
                let v = inst.arguments[i].clone();

                if let Value::Symbol(mut symb) = v.clone()
                {
                    if symb.datatype.raw_type == NonPtrType::Unknown
                    {
                        symb.datatype = DataType::new(NonPtrType::I32, 0, false);
                        inst.arguments[i] = Value::Symbol(symb);
                    }
                }

                if let Value::Literal(mut lit) = v.clone()
                {
                    if lit.datatype.raw_type == NonPtrType::Unknown
                    {
                        lit.datatype = DataType::new(NonPtrType::I32, 0, false);
                        inst.arguments[i] = Value::Literal(lit);
                    }
                }
            }
        }
    }
    func
}