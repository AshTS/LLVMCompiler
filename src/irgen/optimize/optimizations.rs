use crate::irgen::{Function, Value, OpCode};
use crate::irgen::get_value_type;

pub fn optimize_function(f: Function, level: usize, combine: bool) -> Function
{
    let mut func = f.clone();

    func = optimization_clean_branches(func);
    func = optimization_remove_nop(func);

    let mut last_loop = false;

    loop 
    {
        let last = func.clone();

        // Level 2 Optimizations (Clean Register Usage)
        if level >= 2
        {
            // func = optimization_multiple_clean_registers(func);
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

        // Level 0 Optimizations (Constant Folding, Clean Branches, Remove Unused Registers, Remove Dead Code, Remove Unused Labels, Remove Nop's)
        func = optimization_arithmatic_constants(func);
        func = optimization_remove_unused_registers(func);
        func = optimization_remove_nop(func);
        func = optimization_redundant_moves(func);
        func = optimization_remove_nop(func);
        func = optimization_dead_code(func);
        func = optimization_remove_nop(func);
        func = optimization_redundant_labels(func);
        func = optimization_remove_unused_labels(func);
        func = optimization_remove_nop(func);

        // If the code has changed length, keep going
        if func.instructions.len() == last.instructions.len()
        {
            if last_loop
            {
                break;
            }
            
            last_loop = true;
        }
        else
        {
            last_loop = false;
        }
    }

    // If the combine register flag is set, combine the domains of registers
    if combine
    {
        func = optimization_combine_domains(func);
        func = optimize_function(func, level, false);
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
        // Move any labels on nop's
        if indexes_to_remove.contains(&i)
        {
            // Move the labels
            if let Some(mut labels)= func.labels.remove(&i)
            {
                if let Some(next_labels) = func.labels.get(&(i + 1))
                {
                    for l in next_labels
                    {
                        labels.push(l.clone());
                    }
                }

                func.labels.insert(i + 1, labels);
            }
        }

        // If an instruction should be shifted
        if amt_to_shift != 0
        {
            // Move the instruction
            let inst = func.instructions.remove(&i).unwrap();
            func.instructions.insert(i - amt_to_shift, inst);

            // Move the labels
            if let Some(labels)= func.labels.remove(&i)
            {
                for label in &labels.clone()
                {
                    let last = func.labels_reverse.remove(label).unwrap();
                    func.labels_reverse.insert(label.clone(), last - amt_to_shift);
                }

                func.labels.insert(i - amt_to_shift, labels.clone());
            }
        }

        // Increment the amount to shift if need be
        if indexes_to_remove.contains(&i)
        {
            amt_to_shift += 1;
        }
    }

    func.clean_reverse_labels();

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

                // If both branches branch to the same point, make the instruction a jump
                if &vs[0] == &vs[1]
                {
                    let v = func.instructions.get(&index).unwrap().arguments[2].clone();
                    func.instructions.get_mut(&index).unwrap().arguments = vec![v];
                    func.instructions.get_mut(&index).unwrap().opcode = OpCode::Jmp;
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
            // Do not remove return
            if let Some(inst) = func.instructions.get(&index)
            {
                if inst.opcode == OpCode::Ret
                {
                    continue;
                }
            }

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

/// Replace constants and symbols on registers with only one write
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

                    if write_inst.opcode == OpCode::Mov
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

                        else if let Value::Symbol(symb) = &write_inst.arguments[1]
                        {
                            // Copy the symbol
                            let val = symb.clone();

                            // Extract the reads for the symbol
                            let (_, val_reads) = func.get_reads_writes_for(Value::Symbol(val.clone()));

                            // Indexes of instructions to not overwrite
                            let mut disallow = vec![];
                            // Indexes of instructions reachable from the write
                            let mut explored = vec![];

                            // Go over each path
                            for path in func.get_paths_from(writes[0])
                            {
                                // If the path contains a read
                                for r in &reads
                                {
                                    if path.contains(r)
                                    {
                                        let mut spoiled = false;

                                        // Go through the path
                                        for p in path
                                        {
                                            // Fill the explored vector
                                            if !explored.contains(&p)
                                            {
                                                explored.push(p);
                                            }

                                            // Append to the disallowed vector if the values have been spoiled
                                            if spoiled && !disallow.contains(&p)
                                            {
                                                disallow.push(p);
                                            }
                                            if val_reads.contains(&p)
                                            {
                                                spoiled = true;
                                            }
                                        }
                                        break;
                                    }
                                }
                            }

                            // Go over every read for the variable
                            for read in reads
                            {
                                // If it can be reached, but never through a write to the original variable, replace it
                                if explored.contains(&read) && !disallow.contains(&read)
                                {
                                    let mut new_arguments = vec![];

                                    for arg in &func.instructions.get(&read).unwrap().arguments
                                    {
                                        if *arg == Value::Symbol(symbol.clone())
                                        {
                                            new_arguments.push(Value::Symbol(val.clone()));
                                        }
                                        else
                                        {
                                            new_arguments.push(arg.clone());
                                        }
                                    }

                                    func.instructions.get_mut(&read).unwrap().arguments = new_arguments;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    func
}
/* ========= Does not yet function as intended, will require a reverse trace back through the instructions
/// Replace constants and symbols on registers with multiple writes
pub fn optimization_multiple_clean_registers(f: Function) -> Function
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

        for write in &writes
        {
            if !func.arguments.contains(&(symbol.title.clone(), symbol.datatype.clone()))
            {
                if let Some(write_inst) = func.instructions.get(&write)
                {
                    if write_inst.opcode == OpCode::Deref
                    {
                        continue;
                    }

                    if write_inst.opcode == OpCode::Mov
                    {
                        if let Value::Literal(val) = write_inst.arguments[1]
                        {
                            // Not yet implemented
                        }
                        else if let Value::Symbol(symb) = &write_inst.arguments[1]
                        {
                            // Copy the symbol
                            let val = symb.clone();

                            // Extract the reads for the symbol
                            let (_, val_reads) = func.get_reads_writes_for(Value::Symbol(val.clone()));

                            // Indexes of instructions to not overwrite
                            let mut disallow = vec![];
                            // Indexes of instructions reachable from the write
                            let mut explored = vec![];

                            // Go over each path
                            for path in func.get_paths_from(*write)
                            {
                                // If the path contains a read
                                for r in &reads
                                {
                                    if path.contains(&r)
                                    {
                                        let mut spoiled = false;

                                        // Go through the path
                                        for p in path
                                        {
                                            // Stop if a new write is reached
                                            if p != *write && writes.contains(&p)
                                            {
                                                break;
                                            }

                                            // Fill the explored vector
                                            if !explored.contains(&p)
                                            {
                                                explored.push(p);
                                            }

                                            // Append to the disallowed vector if the values have been spoiled
                                            if spoiled && !disallow.contains(&p)
                                            {
                                                disallow.push(p);
                                            }
                                            if val_reads.contains(&p)
                                            {
                                                spoiled = true;
                                            }
                                        }
                                        break;
                                    }
                                }
                            }

                            // Go over every read for the variable
                            for read in &reads
                            {
                                // If it can be reached, but never through a write to the original variable, replace it
                                if explored.contains(&read) && !disallow.contains(&read)
                                {
                                    let mut new_arguments = vec![];

                                    for arg in &func.instructions.get(&read).unwrap().arguments
                                    {
                                        if *arg == Value::Symbol(symbol.clone())
                                        {
                                            new_arguments.push(Value::Symbol(val.clone()));
                                        }
                                        else
                                        {
                                            new_arguments.push(arg.clone());
                                        }
                                    }

                                    func.instructions.get_mut(&read).unwrap().arguments = new_arguments;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    func
}*/

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

        let (reads, writes) = func.get_reads_writes_for(Value::Symbol(symbol.clone()));
        /*
        // Remove Unused Symbols
        if reads.len() == 0
        {
            for index in &writes
            {
                if !func.has_side_effects(*index)
                {
                    func.change_to_nop(*index);
                }
            }
        }*/

        // Remove any writes which are never read
        for write in &writes
        {
            if !func.has_side_effects(*write)
            {
                let explored = &func.get_explored_from(*write)[1..];
                /*
                println!("For: {}", &func.instructions.get(write).unwrap());

                println!("Explored: {:?}", explored);
                println!("Reads: {:?}", reads);*/

                let mut found_read = false;
                for r in &reads
                {
                    if explored.contains(r)
                    {
                        found_read = true;
                        break;
                    }
                }

                // println!("{}\n\n", found_read);

                if !found_read
                {
                    func.change_to_nop(*write);
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
                        func.change_to_nop(writes[0]);
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
        if set.len() == 0
        {
            continue;
        }

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

/// Remove redundant moves
pub fn optimization_redundant_moves(f: Function) -> Function
{
    let mut func = f.clone();

    for (i, instruction) in &func.instructions.clone()
    {
        if instruction.opcode == OpCode::Mov && instruction.arguments[0] == instruction.arguments[1] 
        {
            func.change_to_nop(*i);
        }
    }  

    func
}

/// Remove redundant labels
pub fn optimization_redundant_labels(f: Function) -> Function
{
    let mut func = f.clone();

    // Loop over every instruction
    for i in 0..func.instructions.len()
    {
        if let Some(inst) = func.instructions.get(&i)
        {
            let mut new_args = vec![];
            let mut flag = false;

            // Go over each argument
            for arg in &inst.arguments
            {
                // If the argument is a label, get the first label from the labels
                // pointing to the destination line
                if let Value::Label(label) = arg
                {
                    if let Some(target) = func.labels_reverse.get(label)
                    {
                        if let Some(labels) = func.labels.get(target)
                        {
                            new_args.push(Value::Label(labels[0].clone()));
                            flag = true;
                        }
                    }
                }
                // Otherwise just use the original argument
                else
                {
                    new_args.push(arg.clone());
                }
            }

            // If a change has been made, write the new argument
            if flag
            {
                func.instructions.get_mut(&i).unwrap().arguments = new_args;
            }
        }
    }

    func
}

/// Perform arithmatic operations on constants
pub fn optimization_arithmatic_constants(f: Function) -> Function
{
    let mut func = f.clone();

    for (i, instruction) in &func.instructions.clone()
    {
        if instruction.arguments.len() > 2
        {
            if let Value::Literal(lit0) = instruction.arguments[1]
            {
                if let Value::Literal(lit1) = instruction.arguments[2]
                {
                    match instruction.opcode
                    {
                        OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div =>
                        {
                            let mut new_inst = instruction.clone();

                            new_inst.opcode = OpCode::Mov;
                            if let Value::Literal(mut new_arg) = new_inst.arguments[1].clone()
                            {
                                new_arg.value = 
                                match instruction.opcode
                                {
                                    OpCode::Add => lit0.value + lit1.value,
                                    OpCode::Sub => lit0.value - lit1.value,
                                    OpCode::Mul => lit0.value * lit1.value,
                                    OpCode::Div => lit0.value / lit1.value,
                                    _ => {panic!()}
                                };

                                new_inst.arguments = vec![new_inst.arguments[0].clone(), Value::Literal(new_arg)];

                                func.instructions.insert(*i, new_inst);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }  

    func
}