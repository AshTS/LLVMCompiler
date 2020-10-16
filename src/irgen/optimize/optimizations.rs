use crate::irgen::{Function, Value, OpCode};
use crate::irgen::get_value_type;

pub fn optimize_function(f: Function, level: usize, combine: bool) -> Function
{
    let mut func = f.clone();

    loop 
    {
        let last = func.clone();

        // Level 2 Optimizations (Clean Register Usage)
        if level >= 2
        {
            func = optimization_clean_registers(func);
            func = optimization_remove_nop(func);
        }

        // Level 1 Optimizations (Remove Casts and Jump Chaining)
        if level >= 1
        {
            func = optimization_remove_casts(func);
            func = optimization_remove_nop(func);
            func = optimization_jump_chaining(func);
            func = optimization_remove_nop(func);
        }

        // Level 0 Optimizations (Clean Branches, Remove Unused Registers, Remove Dead Code, Remove Unused Labels, Remove Nop's)
        func = optimization_remove_nop(func);
        func = optimization_clean_branches(func);
        func = optimization_remove_unused_registers(func);
        func = optimization_remove_nop(func);
        func = optimization_dead_code(func);
        func = optimization_remove_nop(func);
        func = optimization_remove_unused_labels(func);
        func = optimization_remove_nop(func);

        // If the code has changed length, keep going
        if func.instructions.len() == last.instructions.len()
        {
            break;
        }
    }

    // If the combine register flag is set, combine the domains of registers
    if combine
    {
        func = optimization_combine_domains(func);
    }

    func.clone()
}

/// Remove nop instructions
pub fn optimization_remove_nop(f: Function) -> Function
{
    let mut func = f.clone();

    let mut indexes_to_remove = vec![];

    // Find all nop commands
    for (index, inst) in (&func.instructions).clone()
    {
        if inst.opcode == OpCode::Nop
        {
            indexes_to_remove.push(index);
        }
    }

    // Keep a record of how much to shift everything by
    let mut amt_to_shift = 0;

    // Go over all instructions
    for i in 0..func.instructions.len()
    {
        // If an instruction should be shifted
        if amt_to_shift != 0
        {
            // Move the instruction
            let inst = func.instructions.remove(&i).unwrap();
            func.instructions.insert(i - amt_to_shift, inst);

            // Move the labels
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

        // Increment the amount to shift if need be
        if indexes_to_remove.contains(&i)
        {
            amt_to_shift += 1;
        }
    }

    func.clone()
}

/// Connect the ends of a chain of jumps
pub fn optimization_jump_chaining(f: Function) -> Function
{
    let mut func = f.clone();

    // Iterate over all instructions
    for (index, inst) in (&func.instructions).clone()
    {
        // If the instruction is a jump
        if inst.opcode == OpCode::Jmp
        {
            if let Some(vs) = func.get_jump_values(index)
            {
                let v = vs[0];

                // Remove the instruction if it jumps to the next instruction
                if v == index + 1
                {
                    func.change_to_nop(index);
                }
                // Chain to the next jump if the instruction jumped to is in turn a jump
                else if let Some(next) = func.instructions.get(&v)
                {
                    if next.opcode == OpCode::Jmp
                    {
                        func.instructions.get_mut(&index).unwrap().arguments[0] = next.arguments[0].clone();
                    }
                }
            }
        }

        // Handle Branches
        else if let Some(vs) = func.get_jump_values(index)
        {
            if vs.len() == 2
            {
                // If the next instruction for the true branch is a jump, just set the label that jump points to as the branch
                if let Some(next) = func.instructions.get(&vs[0])
                {
                    if next.opcode == OpCode::Jmp
                    {
                        func.instructions.get_mut(&index).unwrap().arguments[2] = next.arguments[0].clone();
                    }
                } 

                // If the next instruction for the false branch is a jump, just set the label that jump points to as the branch
                if let Some(next) = func.instructions.get(&vs[1])
                {
                    if next.opcode == OpCode::Jmp
                    {
                        func.instructions.get_mut(&index).unwrap().arguments[3] = next.arguments[0].clone();
                    }
                } 
            }
        }
    }

    func
}

/// Remove unused labels
pub fn optimization_remove_unused_labels(f: Function) -> Function
{
    let mut func = f.clone();
    let mut labels = vec![];

    // Iterate over all instructions, and determine all labels referenced in commands
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

    // Go over all labels stored and if they weren't used, mark them for removal
    let mut labels_to_remove = vec![];
    
    for l in func.labels_reverse.keys()
    {

        if !labels.contains(l)
        {
            labels_to_remove.push(l.clone());
        }
    }

    // Call the remove label function if the label is to be removed
    for l in labels_to_remove
    {
        func.remove_label(l.clone());
    }

    func
}

/// Remove dead code
pub fn optimization_dead_code(f: Function) -> Function
{
    let mut func = f.clone();

    // Get all instructions which can be reached from the start of the function
    let explored = func.get_explored_from(0);

    // Go over all instructions, if it can't be reached, remove it
    for index in 0..func.instructions.len()
    {
        if !explored.contains(&index)
        {
            func.change_to_nop(index);
        }
    }

    func
}

/// Remove casts
pub fn optimization_remove_casts(f: Function) -> Function
{
    let mut func = f.clone();

    let symbols = func.get_all_symbols();

    // Go over all symbols
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
                // If the only write is a cast
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

/// Replace constants written to registers
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
                    if write_inst.opcode == OpCode::Deref
                    {
                        continue;
                    }
                    
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

/// Remove any unused registers
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

        // Skip if the symbol is an argument
        if func.arguments.contains(&(symbol.title.clone(), symbol.datatype.clone()))
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

/// Clean up branches (change a compare and a branch to just a compare)
pub fn optimization_clean_branches(f: Function) -> Function
{
    let mut func = f.clone();

    let symbols = func.get_all_symbols();

    for symbol in symbols
    {
        // Skip the register if the datatype is a reference
        if symbol.datatype.is_ref
        {
            continue;
        }

        let (reads, writes) = func.get_reads_writes_for(Value::Symbol(symbol.clone()));
        
        if reads.len() == 1 && writes.len() == 1 && reads[0] == writes[0] + 1
        {
            // Get the opcode for the proper branch
            let write_inst = func.instructions.get(&writes[0]).unwrap();
            let result_branch = match write_inst.opcode
            {
                OpCode::Ceq => Some(OpCode::Beq),
                OpCode::Cne => Some(OpCode::Bne),
                OpCode::Clt => Some(OpCode::Blt),
                OpCode::Cle => Some(OpCode::Ble),
                OpCode::Cgt => Some(OpCode::Bgt),
                OpCode::Cge => Some(OpCode::Bge),
                _ => None
            };

            if result_branch.is_none() {continue;}

            let mut read_inst = func.instructions.get(&reads[0]).unwrap().clone();

            // If the branch is of the proper form, replace the instruction
            if result_branch.is_some() && read_inst.opcode == OpCode::Bne
            {
                let val = read_inst.arguments[1].clone();

                if let Value::Literal(lit) = val
                {
                    if lit.value == 0
                    {
                        read_inst.opcode = result_branch.unwrap();
                        read_inst.arguments[0] = write_inst.arguments[1].clone();
                        read_inst.arguments[1] = write_inst.arguments[2].clone();

                        func.instructions.insert(reads[0], read_inst);
                    }
                }
            }
        }
    }

    func
}

/// Combine registers which domains which do not overlap
pub fn optimization_combine_domains(f: Function) -> Function
{
    let mut func = f.clone();
    let symbols = func.get_all_symbols();

    let mut domains: Vec<(usize, Value, Vec<usize>)> = vec![];

    let mut to_combine = vec![];

    // Find all domains
    for symbol in symbols
    {
        let domain = func.get_register_domain(Value::Symbol(symbol.clone()));

        domains.push((domain.len(), Value::Symbol(symbol.clone()), domain.clone()));
    }

    // Sort the domains shortest to largest
    domains.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut current = vec![];
    let mut current_domain = vec![];

    // Go over each domain
    for (_, val, items) in domains
    {
        // Make sure there is a loaded domain
        if current.len() == 0
        {
            current.push(val);
            current_domain = items.clone();
        }
        else
        {
            // If the value types are the same
            let mut flag = !(get_value_type(&val) == get_value_type(&current[0]));

            // And the domains don't overlap
            if !flag
            {
                for d in &items
                {
                    if current_domain.contains(&d)
                    {
                        flag = true;

                        break;
                    }
                }
            }

            // The mark the two as being able to be combined
            if !flag
            {
                current.push(val);
                
                for d in &items
                {
                    current_domain.push(*d);
                }
            }
            // Otherwise, move onto the next register
            else
            {
                to_combine.push(current);
                current = vec![val.clone()];
                current_domain = items.clone();
            }
        }
    }

    to_combine.push(current);

    // Go over all register to combine
    for set in to_combine
    {
        let root = set[0].clone();
        for v in &set[1..set.len()]
        {
            for i in 0..func.instructions.len()
            {
                // Replace all occurences of one of the later registers with the root one
                if let Some(inst) = func.instructions.get_mut(&i)
                {
                    let mut new_args = vec![];

                    for arg in &inst.arguments
                    {
                        if arg == v
                        {
                            new_args.push(root.clone());
                        }
                        else
                        {
                            new_args.push(arg.clone());
                        }
                    }

                    inst.arguments = new_args;
                }
            }
        }
    }

    func
}