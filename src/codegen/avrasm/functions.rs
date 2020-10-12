use crate::cli::Error;
use crate::irgen::{Function, OpCode, Value, Symbol};

use super::{generate_comment, generate_label, generate_command, get_label};

use crate::llvm::{NonPtrType, DataType};

use std::collections::HashMap;

pub struct FunctionGenerationContext
{
    free_registers: Vec<usize>,
    function: Function,
    symbol_map: HashMap<String, usize>,
    temp_reg: usize
}

pub fn get_size_datatype(t: DataType) -> usize
{
    if t.num_ptr == 0
    {
        match t.raw_type
        {
            NonPtrType::I8 | NonPtrType::U8 => 1,
            NonPtrType::I16 | NonPtrType::U16 => 2,
            _ => unimplemented!()
        }
    }
    else
    {
        2
    }
}

impl FunctionGenerationContext
{
    pub fn new(function: Function) -> Self
    {
        Self
        {
            function,
            free_registers: vec![25, 23, 22, 21, 20, 19, 18, 17],
            symbol_map: HashMap::new(),
            temp_reg: 16
        }
    }

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
                if self.free_registers[i] % 2 == 0
                {
                    if self.free_registers.contains(&(self.free_registers[i] + 1))
                    {
                        let v = self.free_registers[i];

                        println!("{:?}, {}, {}", self.free_registers, i, v);

                        let j = self.free_registers.iter().position(|&x| x == self.free_registers[i] + 1).unwrap();

                        self.free_registers.remove(i);
                        self.free_registers.remove(j);

                        return Ok(v)
                    }
                }
            }

            Err(Error::error("No more registers available"))
        }
    }

    pub fn get_register(&mut self, symb: &Symbol) -> Result<usize, Error>
    {
        if self.symbol_map.contains_key(&symb.title)
        {
            Ok(self.symbol_map.get(&symb.title).unwrap().clone())
        }
        else
        {
            if get_size_datatype(symb.datatype) == 1
            {
                let reg = self.get_u8_reg()?;

                self.symbol_map.insert(symb.title.clone(), reg.clone());

                Ok(reg)
            }
            else if get_size_datatype(symb.datatype) == 2
            {
                let reg = self.get_u16_reg()?;

                self.symbol_map.insert(symb.title.clone(), reg.clone());

                Ok(reg)
            }
            else
            {
                unimplemented!()
            }
        }
    }

    pub fn move_instruction(&mut self, target: &Value, value: &Value, force_move: bool) -> Result<String, Error>
    {
        match value
        {
            Value::Label(_) => {Err(Error::fatal_error("Cannot use label as a value"))},
            Value::Literal(lit) =>
            {
                if let Value::Symbol(symb) = target
                {
                    if !symb.datatype.is_ref || force_move
                    {
                        let mut result = generate_command(&format!("ldi r{}, {}", self.get_register(symb)?, lit.value & 0xFF))?;

                        if get_size_datatype(lit.datatype) == 2
                        {
                            result += &generate_command(&format!("ldi r{}, {}", self.get_register(symb)? + 1, (lit.value & 0xFF00) >> 8))?;
                        }

                        Ok(result)
                    }
                    else
                    {
                        let mut result = String::new();

                        let reg = self.get_register(symb)?;
                        
                        result += &generate_command(&format!("mov r26, r{}", reg))?;
                        result += &generate_command(&format!("mov r27, r{}", reg + 1))?;
                        result += &generate_command(&format!("ldi r16, {}", lit.value & 0xFF))?;

                        if get_size_datatype(lit.datatype) == 1
                        {
                            result += &generate_command("st X, r16")?;
                        }
                        else if get_size_datatype(lit.datatype) == 2
                        {
                            result += &generate_command(&format!("ldi r16, {}", (lit.value & 0xFF00) >> 8))?;
                            result += &generate_command("st +X, r16")?;
                        }

                        Ok(result)
                    }
                }
                else
                {
                    unimplemented!()
                }
            },
            Value::Symbol(_symb) =>
            {
                unimplemented!()
            },
        }
    }

    pub fn render_function(&mut self) -> Result<String, Error>
    {
        let mut result = String::new();

        // Add the comment at the top of the function
        result += &generate_comment(&format!("Function {}", self.function.render_signature()))?;

        // Add the label marking the start of the function
        result += &generate_label(&format!("f{}", self.function.name))?;

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
                OpCode::Nop => {result += &generate_command("nop")?;},
                OpCode::Jmp =>
                {
                    // Because the label is within the function, we will assume it is just a relative jump

                    if let Value::Label(label) = &inst.arguments[0]
                    {
                        result += &generate_command(&format!("rjmp {}", get_label(&self.function, label)?))?;
                    }
                },
                OpCode::Deref =>
                {
                    result += self.move_instruction(&inst.arguments[0], &inst.arguments[1], true)?.as_str();
                },
                OpCode::Mov =>
                {
                    result += self.move_instruction(&inst.arguments[0], &inst.arguments[1], false)?.as_str();
                },
                _ => {panic!("Not yet implemented conversion for\n{}", inst)
                }
            }
        }

        Ok(result)
    }
}

