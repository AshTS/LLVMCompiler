use std::fmt;
use std::collections::HashMap;
use std::cell::RefCell;

use crate::llvm::{DataType, NonPtrType};

use crate::parser::ParseTreeNode;

use crate::llvm::{expected_got_error};
use crate::llvm::{identifier_from_parse_tree, type_from_parse_tree, arguments_from_parse_tree};

use super::{Statement, get_value_type};

use crate::cli::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode
{
    Alloc,
    Ret,
    Nop,
    Jmp,
    Mov,
    Cne, // Compare Not Equal
    Ceq, // Compare Equal
    Clt, // Compare Less Than
    Cgt, // Compare Greater Than
    Cle, // Compare Less than or Equal
    Cge, // Compare Greater than or Equal
    Bne, // Branch Not Equals
    Beq, // Branch Equals
    Blt,
    Bgt,
    Ble,
    Bge,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl, // Shift Left
    Shr, // Shift Right
    And,
    Or,
    Xor,
    Cast,
    Deref,
    Ref,
    Array,
    Push,
    Call
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol
{
    pub title: String,
    pub datatype: DataType
}

impl Symbol
{
    pub fn new(title: String, datatype: DataType) -> Self
    {
        Symbol
        {
            title,
            datatype
        }
    }
}

impl fmt::Display for Symbol
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "%{} ({})", self.title, self.datatype)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Literal
{
    pub value: i128,
    pub datatype: DataType
}

impl Literal
{
    pub fn new(value: i128, datatype: DataType) -> Self
    {
        Literal
        {
            value,
            datatype
        }
    }
}

impl fmt::Display for Literal
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{} ({})", self.value, self.datatype)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value
{
    Symbol(Symbol),
    Label(String),
    Literal(Literal)
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Value::Symbol(symb) => write!(f, "{}", symb),
            Value::Label(s) => write!(f, "{}", s),
            Value::Literal(lit) => write!(f, "{}", lit)
        }
    }
}
#[derive(Debug, Clone)]
pub struct Instruction
{
    pub opcode: OpCode,
    pub arguments: Vec<Value>
}

impl Instruction
{
    pub fn new(opcode: OpCode, arguments: Vec<Value>) -> Self
    {
        Self
        {
            opcode,
            arguments
        }
    }
}

impl fmt::Display for Instruction
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:<7}", format!("{:?}", self.opcode).to_lowercase())?;

        for arg in &self.arguments
        {
            write!(f, "{:<15}", format!("{}", arg))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Function
{
    pub instructions: HashMap<usize, Instruction>,
    pub labels: HashMap<usize, Vec<String>>,
    pub labels_reverse: HashMap<String, usize>,

    pub symbol_table: HashMap<String, Symbol>,

    pub return_type: DataType,
    pub name: String,
    pub arguments: Vec<(String, DataType)>,

    next_label: usize,
    next_register: usize,
    next_index: usize,

    continue_stack: Vec<String>,
    break_stack: Vec<String>,

    pub return_value: Value
}

impl Function
{
    pub fn new() -> Self
    {
        Self
        {
            instructions: HashMap::new(),
            labels: HashMap::new(),
            labels_reverse: HashMap::new(),

            symbol_table: HashMap::new(),

            return_type: DataType::new(NonPtrType::Void, 0, false),
            name: String::from("[UNKNOWN]"),
            arguments: vec![],

            next_label: 0,
            next_register: 1,
            next_index: 0,

            continue_stack: vec![],
            break_stack: vec![],

            return_value: Value::Symbol(Symbol::new(String::from("R0"), DataType::new(NonPtrType::Void, 0, false)))
        }
    }

    pub fn from_parse_tree_node(node: ParseTreeNode) -> Result<Self, Error>
    {
        match node
        {
            ParseTreeNode::Function(children) =>
            {
                let mut result = Self::new();

                let name = identifier_from_parse_tree(children[1].clone())?;
                let return_type = type_from_parse_tree(children[0].clone())?;
                let arguments = arguments_from_parse_tree(children[2].clone())?;

                result.set_function_signature(return_type, name, arguments);

                let refcell = RefCell::new(&mut result);

                refcell.borrow_mut().return_value = Value::Symbol(Symbol::new(String::from("R0"), return_type.clone()));

                let statement = Statement::from_parse_tree_node(children[3].clone(), &refcell)?;

                statement.render(&refcell)?;

                // Add the exit label
                refcell.borrow_mut().place_label_here(String::from("exit"));
                let ret_val = refcell.borrow().return_value.clone();
                refcell.borrow_mut().add_instruction(Instruction::new(OpCode::Ret, vec![ret_val]));

                let finalresult = refcell.borrow_mut().clone();
                Ok(finalresult)

            },
            default =>
            {
                expected_got_error("Function", default)
            }
        }
    }

    pub fn set_function_signature(&mut self, return_type: DataType, name: String, arguments: Vec<(String, DataType)>)
    {
        self.return_type = return_type;
        self.name = name;
        self.arguments = arguments;

        for (s, t) in &self.arguments
        {
            self.symbol_table.insert(s.clone(), Symbol::new(s.clone(), t.clone()));
        }
    }

    pub fn get_jump_values(&self, index: usize) -> Option<Vec<usize>>
    {
        let inst = self.instructions.get(&index);
        let mut result = vec![];

        if inst.is_some()
        {
            if inst.unwrap().opcode != OpCode::Call
            {
                for val in &inst.unwrap().arguments
                {
                    match val
                    {
                        Value::Label(s) =>
                        {
                            let v = *self.labels_reverse.get(s).unwrap();

                            result.push(v);
                        },
                        _ => {}
                    }
                }
            }
        }

        if result.len() > 0
        {
            Some(result)
        }
        else
        {
            None
        }
    }

    pub fn change_to_nop(&mut self, index: usize)
    {
        self.instructions.get_mut(&index).unwrap().opcode = OpCode::Nop;
        self.instructions.get_mut(&index).unwrap().arguments = vec![];
    }

    pub fn get_next_branches(&self, index: usize) -> Vec<usize>
    {
        let mut result = vec![];

        if index < self.instructions.len() - 1 && self.instructions.get(&index).unwrap().opcode != OpCode::Jmp
        {
            result.push(index + 1);
        }

        let inst = self.instructions.get(&index);

        if inst.is_some()
        {
            if inst.unwrap().opcode != OpCode::Call
            {
                for val in &inst.unwrap().arguments
                {
                    match val
                    {
                        Value::Label(s) =>
                        {
                            let v = *self.labels_reverse.get(s).unwrap();

                            if !result.contains(&v)
                            {
                                result.push(v);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        result
    } 

    pub fn place_label(&mut self, label: String, index: usize)
    {
        if self.labels.contains_key(&index)
        {
            self.labels.get_mut(&index).unwrap().push(label.clone());
        }
        else
        {
            self.labels.insert(index, vec![label.clone()]);
        }

        self.labels_reverse.insert(label.clone(), index);
    }

    pub fn place_label_here(&mut self, label: String)
    {
        self.place_label(label, self.next_index);
    }

    pub fn get_label(&mut self) -> String
    {
        self.next_label += 1;

        format!("L{}", self.next_label - 1)
    }

    pub fn get_label_and_place(&mut self) -> String
    {
        let label = self.get_label();
        self.place_label(label.clone(), self.next_index);

        label.clone()
    }

    pub fn get_register(&mut self) -> String
    {
        self.next_register += 1;

        format!("R{}", self.next_register - 1)
    }

    pub fn add_instruction(&mut self, inst: Instruction)
    {
        self.instructions.insert(self.next_index, inst);
        self.next_index += 1;
    }

    pub fn enter_loop(&mut self) -> (String, String)
    {
        let entry = self.get_label();
        let exit = self.get_label();

        self.continue_stack.push(entry.clone());
        self.break_stack.push(exit.clone());

        (entry, exit)
    }

    pub fn exit_loop(&mut self)
    {
        self.continue_stack.pop();
        self.break_stack.pop();
    }

    pub fn get_continue(&mut self) -> Option<String>
    {
        if self.continue_stack.len() > 0
        {
            Some(self.continue_stack[self.continue_stack.len() - 1].clone())
        }
        else
        {
            None
        }
    }

    pub fn get_break(&mut self) -> Option<String>
    {
        if self.break_stack.len() > 0
        {
            Some(self.break_stack[self.continue_stack.len() - 1].clone())
        }
        else
        {
            None
        }
    }

    pub fn get_explored_from(&self, index: usize) -> Vec<usize>
    {
        let mut fronts= vec![index];
        let mut explored = vec![];

        // While there are still instructions to explore
        while fronts.len() > 0
        {
            let mut next_fronts = vec![];

            for front in fronts
            {
                explored.push(front);

                for v in self.get_next_branches(front)
                {
                    if !explored.contains(&v) && !next_fronts.contains(&v)
                    {
                        next_fronts.push(v);
                    }
                }
            }

            fronts = next_fronts;
        }

        explored
    }

    pub fn has_side_effects(&self, index: usize) -> bool
    {
        match self.instructions.get(&index)
        {
            Some(v) =>
            {
                v.opcode == OpCode::Call
            },
            None => false
        }
    }

    pub fn get_reads_writes_for(&self, value: Value) -> (Vec<usize>, Vec<usize>)
    {
        let mut reads = vec![];
        let mut writes = vec![];

        for (index, inst) in &self.instructions
        {
            if inst.arguments.len() > 0
            {
                match inst.opcode
                {
                    OpCode::Beq | OpCode::Bge | OpCode::Bgt | OpCode::Ble | OpCode::Blt | OpCode::Bne | OpCode::Push | OpCode::Ret =>
                    {
                        if inst.arguments.contains(&value)
                        {
                            reads.push(*index);
                        }
                    },
                    _ => 
                    {
                        if inst.arguments[0] == value
                        {
                            if !get_value_type(&value).unwrap().is_ref || inst.opcode == OpCode::Cast
                            {
                                writes.push(*index);
                            }
                            else
                            {
                                reads.push(*index);
                            }
                            
                        }
                        
                        if inst.arguments.len() > 1
                        {
                            if inst.arguments[1..inst.arguments.len()].contains(&value)
                            {
                                reads.push(*index);
                            }
                        }
                    }
                }
            }
        }

        (reads, writes)
    }

    pub fn get_all_symbols(&self) -> Vec<Symbol>
    {
        let mut result = vec![];

        for (_, inst) in &self.instructions
        {
            for val in &inst.arguments
            {
                if let Value::Symbol(symbol) = val
                {
                    if !result.contains(symbol)
                    {
                        result.push(symbol.clone());
                    }
                } 
            }
        }

        result
    }

    pub fn render_signature(&self) -> String
    {
        let mut result = String::new();

        result += &format!("{} {}(", self.return_type, self.name);

        for (i, (t, n)) in (&self.arguments).iter().enumerate()
        {
            result += &format!("{} {}", t, n);

            if i != self.arguments.len() - 1
            {
                result += ", ";
            }
        }

        result += ")";

        result
    }

    pub fn remove_label(&mut self, label: String)
    {
        if let Some(index) = self.labels_reverse.get(&label)
        {
            if let Some(labels) = self.labels.get(&index)
            {
                if labels.contains(&label)
                {
                    let mut next_vec = vec![];
                    if let Some(r) = self.labels.get(&index)
                    {
                        for value in r.iter()
                        {
                            if *value != label
                            {
                                next_vec.push(value.clone());
                            }
                        }
                    }

                    if next_vec.len() > 0
                    {
                        self.labels.insert(*index, next_vec);
                    }
                    else
                    {
                        self.labels.remove(index);
                    }
                }
            }
        }

        self.labels_reverse.remove(&label);
    }
}

impl fmt::Display for Function
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{} {}(", self.return_type, self.name)?;

        for (i, (t, n)) in (&self.arguments).iter().enumerate()
        {
            write!(f, "{} {}", t, n)?;

            if i != self.arguments.len() - 1
            {
                write!(f, ", ")?;
            }
        }

        writeln!(f, ")")?;

        for i in 0..self.instructions.len()
        {
            write!(f, "{:03} ", i)?;

            let mut labels_str = String::new();

            if let Some(labels) = self.labels.get(&i)
            {
                for label in labels
                {
                    labels_str += &format!("{}: ", label);
                }
            }

            write!(f, "{:15}", labels_str)?;

            let inst = self.instructions.get(&i);

            if inst.is_none()
            {
                writeln!(f, "[Line Removed]")?;
            }
            else
            {
                writeln!(f, "{}", inst.unwrap())?;
            }
        }

        Ok(())
    }
}
