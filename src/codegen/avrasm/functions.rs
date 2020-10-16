use crate::cli::Error;
use crate::irgen::{Function, OpCode, Value, Symbol};

use super::{generate_comment, generate_label, generate_command, get_label, get_size_datatype};

use std::collections::HashMap;

/// A wrapper for giving a context to code generation for an avrasm function
pub struct FunctionGenerationContext
{
    free_registers: Vec<usize>,
    function: Function,
    symbol_map: HashMap<String, usize>,
    temp_reg: usize,
    last_temp_assignment: String
}

impl FunctionGenerationContext
{
    /// Generate a new FunctionGeneratorContext for the given IR function
    pub fn new(function: Function) -> Self
    {
        Self
        {
            function,
            free_registers: vec![25, 23, 22, 21, 20, 19, 18, 17],
            symbol_map: HashMap::new(),
            temp_reg: 16,
            last_temp_assignment: String::new()
        }
    }

    /// Request a new 8 bit register
    pub fn get_u8_reg(&mut self) -> Result<usize, Error>
    {
        if self.free_registers.len() == 0
        {
            Err(Error::error("No more registers available"))
        }
        else
        {
            Ok(self.free_registers.pop().unwrap())
        }
    }

    /// Request a new 16 bit register
    pub fn get_u16_reg(&mut self) -> Result<usize, Error>
    {
        if self.free_registers.len() == 0
        {
            Err(Error::error("No more registers available"))
        }
        else
        {
            for i in 0..self.free_registers.len()
            {
                // The register must be even
                if self.free_registers[i] % 2 == 0
                {
                    // And have register n + 1 also be available
                    if self.free_registers.contains(&(self.free_registers[i] + 1))
                    {
                        // Remove register n
                        let v = self.free_registers[i];
                        self.free_registers.remove(i);

                        // Remove register n + 1
                        let j = self.free_registers.iter().position(|&x| x == v + 1).unwrap();
                        self.free_registers.remove(j);

                        return Ok(v)
                    }
                }
            }

            Err(Error::error("No more registers available"))
        }
    }

    /// Get the register allocated to the given symbol
    pub fn get_register(&mut self, symb: &Symbol) -> Result<usize, Error>
    {
        // If the symbol map contains the symbol, just return the register previously recorded
        if self.symbol_map.contains_key(&symb.title)
        {
            Ok(self.symbol_map.get(&symb.title).unwrap().clone())
        }
        else
        {
            // Get an 8 bit register if the datatype is 8 bits
            if get_size_datatype(symb.datatype) == 1
            {
                let reg = self.get_u8_reg()?;

                self.symbol_map.insert(symb.title.clone(), reg.clone());

                Ok(reg)
            }
            // Get an 16 bit register if the datatype is 16 bits
            else if get_size_datatype(symb.datatype) == 2
            {
                let reg = self.get_u16_reg()?;

                self.symbol_map.insert(symb.title.clone(), reg.clone());

                Ok(reg)
            }
            // Otherwise, panic because there should never be a size other that these two so far
            else
            {
                panic!()
            }
        }
    }

    /// Add a move instruction between two values (the force_move flag forces a move, ignoring references)
    pub fn move_instruction(&mut self, target: &Value, value: &Value, force_move: bool) -> Result<String, Error>
    {
        match value
        {
            Value::Label(_) => {Err(Error::fatal_error("Cannot use label as a value"))},
            Value::Literal(lit) =>
            {
                // Moving a literal into a symbol
                if let Value::Symbol(symb) = target
                {
                    // Moving a literal into a symbol
                    if !symb.datatype.is_ref || force_move
                    {
                        // Load the low byte into the first register
                        let mut result = generate_command(&format!("ldi r{}, {}", self.get_register(symb)?, lit.value & 0xFF))?;

                        // Load the high byte if needed
                        if get_size_datatype(lit.datatype) == 2
                        {
                            result += &generate_command(&format!("ldi r{}, {}", self.get_register(symb)? + 1, (lit.value & 0xFF00) >> 8))?;
                        }

                        Ok(result)
                    }
                    // Moving a literal into a reference symbol
                    else
                    {
                        let mut result = String::new();

                        let reg = self.get_register(symb)?;
                        
                        // Load the reference into the X index register
                        result += &generate_command(&format!("movw r26, {}", reg))?;

                        let new_temp = format!("{}", lit.value & 0xFF);
                        if self.last_temp_assignment != new_temp
                        {
                            self.last_temp_assignment = format!("{}", lit.value & 0xFF);
                            result += &generate_command(&format!("ldi r16, {}", self.last_temp_assignment))?;
                        }

                        // Store the low byte
                        result += &generate_command("st X, r16")?;

                        // If needed store the high byte
                        if get_size_datatype(lit.datatype) == 2
                        {
                            let new_temp = format!("{}", (lit.value & 0xFF00) >> 8);
                            if self.last_temp_assignment != new_temp
                            {
                                self.last_temp_assignment = new_temp;
                                result += &generate_command(&format!("ldi r16, {}", self.last_temp_assignment))?;
                            }

                            result += &generate_command("st +X, r16")?;
                        }

                        Ok(result)
                    }
                }
                // Moving a literal into a literal (must be a reference)
                else if let Value::Literal(target_lit) = target
                {
                    // Moving a literal into a non-reference literal makes no sense, so just panic
                    if !target_lit.datatype.is_ref || force_move
                    {
                        panic!()
                    }
                    // Moving a literal into a reference literal
                    else
                    {
                        let mut result = String::new();

                        // If the target is within the space which allows in and out commands, make use of those commands
                        if target_lit.value < 0x60 && target_lit.value >= 0x20
                        {
                            let new_temp = format!("{}", lit.value & 0xFF);
                            if self.last_temp_assignment != new_temp
                            {
                                self.last_temp_assignment = format!("{}", lit.value & 0xFF);
                                result += &generate_command(&format!("ldi r16, {}", self.last_temp_assignment))?;
                            }

                            result += &generate_command(&format!("out {}, r16", target_lit.value - 0x20))?;

                            if get_size_datatype(lit.datatype) == 2
                            {
                                return Err(Error::error("Cannot assign 16 bit value to an 8 bit register"))
                            }
                        }
                        // Otherwise the st command will need to be used
                        else
                        {
                            // Load the target literal into the X register
                            result += &generate_command(&format!("ldi r26, {}", target_lit.value & 0xFF))?;
                            result += &generate_command(&format!("ldi r27, {}", (target_lit.value & 0xFF00) >> 8))?;
                            
                            let new_temp = format!("{}", lit.value & 0xFF);
                            if self.last_temp_assignment != new_temp
                            {
                                self.last_temp_assignment = format!("{}", lit.value & 0xFF);
                                result += &generate_command(&format!("ldi r16, {}", self.last_temp_assignment))?;
                            }

                            // Store the low byte
                            result += &generate_command("st X, r16")?;

                            // If needed store the high byte
                            if get_size_datatype(lit.datatype) == 2
                            {
                                let new_temp = format!("{}", (lit.value & 0xFF00) >> 8);
                                if self.last_temp_assignment != new_temp
                                {
                                    self.last_temp_assignment = format!("{}", (lit.value & 0xFF00) >> 8);
                                    result += &generate_command(&format!("ldi r16, {}", self.last_temp_assignment))?;
                                }

                                result += &generate_command("st +X, r16")?;
                            }
                        }

                        Ok(result)
                    }
                }
                // Anything else does not make sense, and therefore panics
                else
                {
                    panic!()
                }
            },
            Value::Symbol(src_symb) =>
            {
                let src_reg = self.get_register(src_symb)?;

                // Moving a symbol into a symbol
                if let Value::Symbol(symb) = target
                {
                    // Move a symbol into another symbol
                    if !symb.datatype.is_ref || force_move
                    {
                        let mut result = String::new();

                        // If the datasize is 8 bits, use the mov command
                        if get_size_datatype(src_symb.datatype) == 1
                        {
                            result += &generate_command(&format!("mov r{}, r{}", self.get_register(symb)?, src_reg))?;
                        }

                        // If the datasize is 16 bits, use the movw command
                        else if get_size_datatype(src_symb.datatype) == 2
                        {
                            result += &generate_command(&format!("movw r{}, r{}", self.get_register(symb)?, src_reg))?;
                        }

                        Ok(result)
                    }
                    // Move a symbol into the memory referenced by a symbol
                    else
                    {
                        let mut result = String::new();

                        let reg = self.get_register(symb)?;
                        
                        // Load the destination into the X register
                        result += &generate_command(&format!("movw r26, {}", reg))?;

                        // Write the low byte
                        result += &generate_command(&format!("st X, r{}", src_reg))?;

                        // Write the high byte if needed
                        if get_size_datatype(src_symb.datatype) == 2
                        {
                            result += &generate_command(&format!("st +X, r{}", src_reg + 1))?;
                        }

                        Ok(result)
                    }
                }
                // Moving a symbol into a literal
                else if let Value::Literal(target_lit) = target
                {
                    // If the target isn't a reference, this move doesn't make sense, so panic
                    if !target_lit.datatype.is_ref || force_move
                    {
                        panic!()
                    }
                    // Moving a symbol into a reference literal
                    else
                    {
                        let mut result = String::new();

                        // If the target is in the domain usable by  the out and in commands, use those commands
                        if target_lit.value < 0x60 && target_lit.value >= 0x20
                        {
                            result += &generate_command(&format!("out {}, r{}", target_lit.value - 0x20, src_reg))?;

                            if get_size_datatype(src_symb.datatype) == 2
                            {
                                return Err(Error::error("Cannot assign 16 bit value to an 8 bit register"))
                            }
                        }
                        else
                        {
                            // Load the target into the X register
                            result += &generate_command(&format!("ldi r26, {}", target_lit.value & 0xFF))?;
                            result += &generate_command(&format!("ldi r27, {}", (target_lit.value & 0xFF00) >> 8))?;

                            // Store the low byte
                            result += &generate_command(&format!("st X, r{}", src_reg))?;

                            // Store the high byte if needed
                            if get_size_datatype(src_symb.datatype) == 2
                            {
                                result += &generate_command(&format!("st +X, r{}", src_reg + 1))?;
                            }
                        }

                        Ok(result)
                    }
                }
                // Anything else does not make sense, and therefore panics
                else
                {
                    panic!()
                }
            },
        }
    }

    /// Add a dereference instruction (effectively like a move, but out of a reference)
    pub fn dereference_instruction(&mut self, target: &Value, value: &Value) -> Result<String, Error>
    {
        match value
        {
            Value::Label(_) => {Err(Error::fatal_error("Cannot use label as a value"))},
            Value::Literal(lit) =>
            {
                // Derefencing a literal into a register
                if let Value::Symbol(symb) = target
                {
                    // If the literal is within the range allowable by the out and in commands use the in command
                    if lit.value < 0x60 && lit.value >= 0x20
                    {
                        let mut result = String::new();
                        let reg = self.get_register(symb)?;

                        if get_size_datatype(symb.datatype) == 1
                        {
                            result += &generate_command(&format!("in r{}, {}", reg, lit.value - 0x20))?;
                        }
                        else if get_size_datatype(symb.datatype) == 2
                        {
                            result += &generate_command(&format!("in r{}, {}", reg + 1, lit.value + 1 - 0x20))?;
                        }

                        Ok(result)
                    }
                    else
                    {
                        let mut result = String::new();
                        let reg = self.get_register(symb)?;

                        // Load the literal into the X register
                        result += &generate_command(&format!("ldi r26, {}", lit.value & 0xFF))?;
                        result += &generate_command(&format!("ldi r27, {}", (lit.value & 0xFF00) >> 8))?;

                        // Load the low byte
                        result += &generate_command(&format!("ld r{}, X", reg))?;

                        // Load the high byte if needed
                        if get_size_datatype(symb.datatype) == 2
                        {
                            result += &generate_command(&format!("ld r{}, +X", reg + 1))?;
                        }

                        Ok(result)
                    }
                }
                // Can't derefernce a literal into a literal, therefore panic
                else
                {
                    panic!()
                }
            },
            Value::Symbol(src_symb) =>
            {
                // Derefernce a symbol into a register
                if let Value::Symbol(symb) = target
                {
                    let mut result = String::new();
                    let reg = self.get_register(symb)?;
                    let src_reg = self.get_register(src_symb)?;

                    // Load the source into the X register
                    result += &generate_command(&format!("movw r26, r{}", src_reg))?;

                    // Load the low byte
                    result += &generate_command(&format!("ld r{}, X", reg))?;

                    // Load the high byte if needed
                    if get_size_datatype(symb.datatype) == 2
                    {
                        result += &generate_command(&format!("ld r{}, +X", reg + 1))?;
                    }

                    Ok(result)
                }
                // Can't derefernce a symbol into a literal, therefore panic
                else
                {
                    panic!()
                }
            }
        }
    }

    /// Add an add instruction
    pub fn add_instruction(&mut self, dest: &Value, v0: &Value, v1: &Value) -> Result<String, Error>
    {
        // Get the destination register
        let dest_reg = if let Value::Symbol(symb) = dest
        {
            self.get_register(&symb)?
        }
        else
        {
            return Err(Error::error("Unable to assign to anything but a symbol"));
        };

        match v0
        {
            Value::Label(_) => {Err(Error::fatal_error("Cannot use label as a value"))},
            Value::Literal(_) =>
            {
                // This should have been cleaned up by the IR gen
                if let Value::Literal(_) = v1
                {
                    Err(Error::error("Add command invoked with two literals"))
                }
                // If having a literal as the first argument can be solved by reversing the order of the operands, do so
                else
                {
                    self.add_instruction(dest, v1, v0)
                }
            },
            Value::Symbol(symb0) =>
            {
                // If it is an '+=' then it can be a simple add (hopefully)
                if dest == v0
                {
                    let mut result = String::new();

                    // Symbol0 = Symbol0 + Symbol1
                    if let Value::Symbol(symb1) = v1
                    {
                        result += &generate_command(&format!("add r{}, r{}", dest_reg, self.get_register(&symb1)?))?;
                        if get_size_datatype(symb0.datatype) == 2
                        {
                            result += &generate_command(&format!("adc r{}, r{}", dest_reg + 1, self.get_register(&symb1)? + 1))?;
                        }
                    }
                    // Symbol0 = Symbol0 + Lit
                    else if let Value::Literal(lit1) = v1
                    {
                        // If the literal is 1, use the increment command instead
                        if lit1.value == 1
                        {
                            result += &generate_command(&format!("inc r{}", dest_reg))?;
                        }
                        else
                        {
                            let new_temp = format!("{}", lit1.value & 0xFF);
                            if self.last_temp_assignment != new_temp
                            {
                                self.last_temp_assignment = new_temp;
                                result += &generate_command(&format!("ldi r16, {}", self.last_temp_assignment))?;
                            }

                            result += &generate_command(&format!("add r{}, r16", dest_reg))?;
                        }
                        
                        // Use the adc command for 16 bit operands
                        if get_size_datatype(symb0.datatype) == 2
                        {
                            let new_temp = format!("{}", (lit1.value & 0xFF00) >> 8);
                            if self.last_temp_assignment != new_temp
                            {
                                self.last_temp_assignment = new_temp;
                                result += &generate_command(&format!("ldi r16, {}", self.last_temp_assignment))?;
                            }

                            result += &generate_command(&format!("adc r{}, r16", dest_reg + 1))?;
                        }
                    }
                    else
                    {
                        return Err(Error::error("Unable to read value from label"));
                    }

                    Ok(result)
                }
                // A standard + (Not yet implemented)
                else
                {
                    unimplemented!()
                }
            },
        }
    }

    // Add a branch operation
    pub fn add_branch(&mut self, inst: &str, v0: &Value, v1: &Value, l0: &Value, l1: &Value) -> Result<String, Error>
    {
        let label0 = if let Value::Label(s) = l0 {s} else {return Err(Error::error("Expected a label"));};
        let label1 = if let Value::Label(s) = l1 {s} else {return Err(Error::error("Expected a label"));};

        if let Value::Symbol(symb0) = v0
        {
            let reg0 = self.get_register(&symb0)?;

            // Handle 8 bit operands
            if get_size_datatype(symb0.datatype) == 1
            {
                let mut result = String::new();

                // Compare a symbol to a symbol
                if let Value::Symbol(symb1) = v1
                {
                    let reg1 = self.get_register(&symb1)?;

                    result += &generate_command(&format!("cp r{}, r{}", reg0, reg1))?;
                }
                // Compare a symbol to a literal
                else if let Value::Literal(lit1) = v1
                {
                    result += &generate_command(&format!("cpi r{}, {}", reg0, lit1.value))?;
                }
                else
                {
                    unimplemented!()
                }

                // Perform the branch and add an 'else' jmp
                result += &generate_command(&format!("{} {}", inst, get_label(&self.function, label0)?))?;
                result += &generate_command(&format!("jmp {}", get_label(&self.function, label1)?))?;

                Ok(result)
            }
            // Handle operands of 16 bits (not yet implemented)
            else if get_size_datatype(symb0.datatype) == 2
            {
                unimplemented!()
            }
            // No support for operands larger than 16 bits
            else
            {
                panic!()
            }
        }
        // Not yet implemented having the first argument be a literal
        else
        {
            unimplemented!()
        }
    }

    /// Render an IR function in AVR Assembly
    pub fn render_function(&mut self) -> Result<String, Error>
    {
        let mut result = String::new();

        // Add the comment at the top of the function
        result += &generate_comment(&format!("Function {}", self.function.render_signature()))?;

        // Add the label marking the start of the function
        result += &generate_label(&format!("f{}", self.function.name))?;

        // Iterate over each instruction (in order)
        for i in 0..self.function.instructions.len()
        {
            let inst = self.function.instructions.get(&i).unwrap().clone();

            // If there are labels available for a given instruction, write those in
            if let Some(labels) = self.function.labels.get(&i)
            {
                for label in labels
                {
                    result += &generate_label(&get_label(&self.function, label)?)?;
                }
            }
            
            match inst.opcode
            {
                OpCode::Nop => {},
                OpCode::Jmp =>
                {
                    // Because the label is within the function, we will assume it is just a relative jump

                    if let Value::Label(label) = &inst.arguments[0]
                    {
                        result += &generate_command(&format!("jmp {}", get_label(&self.function, label)?))?;
                    }
                },

                // Mov Alloc and Cast are all wrappers for moves
                OpCode::Mov | OpCode::Alloc | OpCode::Cast =>
                {
                    result += self.move_instruction(&inst.arguments[0], &inst.arguments[1], false)?.as_str();
                },

                // Dereference
                OpCode::Deref =>
                {
                    result += self.dereference_instruction(&inst.arguments[0], &inst.arguments[1])?.as_str();
                },

                // Add
                OpCode::Add =>
                {
                    result += self.add_instruction(&inst.arguments[0], &inst.arguments[1], &inst.arguments[2])?.as_str();
                },

                // All of the branches

                // Branch Equal
                OpCode::Beq =>
                {
                    result += self.add_branch("breq", &inst.arguments[0], &inst.arguments[1], &inst.arguments[2], &inst.arguments[3])?.as_str();
                },

                // Branch Not Equal
                OpCode::Bne =>
                {
                    result += self.add_branch("brne", &inst.arguments[0], &inst.arguments[1], &inst.arguments[2], &inst.arguments[3])?.as_str();
                },

                // Branch Less Than
                OpCode::Blt =>
                {
                    result += self.add_branch("brlo", &inst.arguments[0], &inst.arguments[1], &inst.arguments[2], &inst.arguments[3])?.as_str();
                },

                // Branch Less Than or Equal To
                OpCode::Ble =>
                {
                    // a <= b == !(b < a)
                    // Reverse Arguments and Branches
                    result += self.add_branch("brlo", &inst.arguments[1], &inst.arguments[0], &inst.arguments[3], &inst.arguments[2])?.as_str();
                },
                OpCode::Bgt =>
                {
                    // a > b == b < a
                    // Reverse Arguments
                    result += self.add_branch("brlo", &inst.arguments[1], &inst.arguments[0], &inst.arguments[2], &inst.arguments[3])?.as_str();
                },
                OpCode::Bge =>
                {
                    // a >= b == !(a < b)
                    // Reverse Branches
                    result += self.add_branch("brlo", &inst.arguments[0], &inst.arguments[1], &inst.arguments[3], &inst.arguments[2])?.as_str();
                },
                _ => {panic!("Not yet implemented conversion for\n{}", inst)
                }
            }
        }

        Ok(result)
    }
}

