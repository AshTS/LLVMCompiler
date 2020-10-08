use crate::irgen::{Function, Instruction, Value, OpCode};

pub fn optimize_function(f: Function) -> Function
{
    let mut func = f.clone();

    loop 
    {
        let last = func.clone();

        func = optimization_remove_nop(func);
        func = optimization_jump_chaining(func);
        func = optimization_remove_nop(func);
        func = optimization_dead_code(func);
        func = optimization_remove_nop(func);

        if func.instructions.len() == last.instructions.len()
        {
            break;
        }
    }

    func.clone()
}

pub fn optimization_remove_nop(f: Function) -> Function
{
    let mut func = f.clone();

    let mut indexes_to_remove = vec![];

    for (index, inst) in (&func.instructions).clone()
    {
        if inst.opcode == OpCode::Nop
        {
            indexes_to_remove.push(index);
        }
    }

    let mut amt_to_shift = 0;

    for i in 0..func.instructions.len()
    {
        if amt_to_shift != 0
        {
            let inst = func.instructions.remove(&i).unwrap();
            func.instructions.insert(i - amt_to_shift, inst);

            let labels = func.labels.remove(&i);

            if labels.is_some()
            {
                for label in &labels.clone().unwrap()
                {
                    let last = func.labels_reverse.remove(label).unwrap();
                    func.labels_reverse.insert(label.clone(), last - amt_to_shift);
                }

                func.labels.insert(i - amt_to_shift, labels.clone().unwrap());
            }
        }

        if indexes_to_remove.contains(&i)
        {
            amt_to_shift += 1;
        }
    }

    func.clone()
}

pub fn optimization_jump_chaining(f: Function) -> Function
{
    let mut func = f.clone();

    for (index, inst) in (&func.instructions).clone()
    {
        if inst.opcode == OpCode::Jmp
        {
            if let Some(v) = func.get_jump_value(index)
            {
                if v == index + 1
                {
                    func.change_to_nop(index);
                }
                else if let Some(next) = func.instructions.get(&v)
                {
                    if next.opcode == OpCode::Jmp
                    {
                        func.instructions.get_mut(&index).unwrap().arguments[0] = next.arguments[0].clone();
                    }
                }
            }
        }
    }

    func
}

pub fn optimization_dead_code(f: Function) -> Function
{
    let mut func = f.clone();

    let explored = func.get_explored_from(0);

    for index in 0..func.instructions.len()
    {
        if !explored.contains(&index)
        {
            func.change_to_nop(index);
        }
    }

    func
}