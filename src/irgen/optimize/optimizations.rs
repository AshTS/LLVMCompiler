use crate::irgen::{Function, Value, OpCode};

pub fn optimize_function(f: Function, level: usize) -> Function
{
    let mut func = f.clone();

    loop 
    {
        let last = func.clone();

        func = optimization_remove_nop(func);

        if level >= 2
        {
            func = optimization_clean_registers(func);
            func = optimization_remove_nop(func);
        }

        if level >= 1
        {
            func = optimization_remove_casts(func);
            func = optimization_remove_nop(func);
            func = optimization_jump_chaining(func);
            func = optimization_remove_nop(func);
        }
        
        func = optimization_remove_unused_registers(func);
        func = optimization_remove_nop(func);
        func = optimization_dead_code(func);
        func = optimization_remove_nop(func);
        func = optimization_remove_unused_labels(func);
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

pub fn optimization_remove_unused_labels(f: Function) -> Function
{
    let mut func = f.clone();
    let mut labels = vec![];

    for (_, inst) in (&func.instructions).clone()
    {
        for v in inst.arguments
        {
            if let Value::Label(label) = v
            {
                labels.push(label);
            }
        }
    }

    let mut labels_to_remove = vec![];
    
    for l in func.labels_reverse.keys()
    {

        if !labels.contains(l)
        {
            labels_to_remove.push(l.clone());
        }
    }

    for l in labels_to_remove
    {
        func.remove_label(l.clone());
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

pub fn optimization_remove_casts(f: Function) -> Function
{
    let mut func = f.clone();

    let symbols = func.get_all_symbols();

    for symbol in symbols
    {
        // Skip if the symbol is an argument
        if func.arguments.contains(&(symbol.title.clone(), symbol.datatype.clone()))
        {
            continue;
        }

        let (reads, writes) = func.get_reads_writes_for(Value::Symbol(symbol.clone()));

        if writes.len() == 1
        {
            if let Some(inst) = func.instructions.get(&writes[0])
            {
                if inst.opcode == OpCode::Cast
                {
                    if let Value::Literal(lit) = inst.arguments[1]
                    {
                        let mut val = lit.clone();

                        val.datatype = symbol.datatype;

                        // Remove the single write
                        func.change_to_nop(writes[0]);

                        // Go through the reads and replace the usages with the written value
                        for index in &reads
                        {
                            let mut new_arguments = vec![];

                            for arg in &func.instructions.get(&index).unwrap().arguments
                            {
                                if *arg == Value::Symbol(symbol.clone())
                                {
                                    new_arguments.push(Value::Literal(val.clone()));
                                }
                                else
                                {
                                    new_arguments.push(arg.clone());
                                }
                            }

                            func.instructions.get_mut(&index).unwrap().arguments = new_arguments;
                        }
                    }
                }
            }
        }
    }

    func
}


pub fn optimization_clean_registers(f: Function) -> Function
{
    let mut func = f.clone();

    let symbols = func.get_all_symbols();

    for symbol in symbols
    {
        // Skip the register if the datatype is a reference (the instruction will have side effects)
        if symbol.datatype.is_ref
        {
            continue;
        }

        let (reads, writes) = func.get_reads_writes_for(Value::Symbol(symbol.clone()));
        
        // Replace Constants
        if writes.len() == 1
        {
            if !func.arguments.contains(&(symbol.title.clone(), symbol.datatype.clone()))
            {
                if let Some(write_inst) = func.instructions.get(&writes[0])
                {
                    if write_inst.arguments.len() == 2
                    {
                        if let Value::Literal(val) = write_inst.arguments[1]
                        {
                            // Remove the single write
                            func.change_to_nop(writes[0]);

                            // Go through the reads and replace the usages with the written value
                            for index in reads
                            {
                                let mut new_arguments = vec![];

                                for arg in &func.instructions.get(&index).unwrap().arguments
                                {
                                    if *arg == Value::Symbol(symbol.clone())
                                    {
                                        new_arguments.push(Value::Literal(val.clone()));
                                    }
                                    else
                                    {
                                        new_arguments.push(arg.clone());
                                    }
                                }

                                func.instructions.get_mut(&index).unwrap().arguments = new_arguments;
                            }
                        }
                    }
                }
            }
        }
    }

    func
}

pub fn optimization_remove_unused_registers(f: Function) -> Function
{
    let mut func = f.clone();

    let symbols = func.get_all_symbols();

    for symbol in symbols
    {
        // Skip the register if the datatype is a reference (the instruction will have side effects)
        if symbol.datatype.is_ref
        {
            continue;
        }

        let (reads, writes) = func.get_reads_writes_for(Value::Symbol(symbol.clone()));
        
        // Remove Unused Symbols
        if reads.len() == 0
        {
            for index in writes
            {
                if !func.has_side_effects(index)
                {
                    func.change_to_nop(index);
                }
            }
        }
    }

    func
}