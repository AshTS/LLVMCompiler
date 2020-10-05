use std::fmt;
use std::collections::HashMap;

use crate::llvm::{DataType, NonPtrType};

use crate::parser::ParseTreeNode;

use crate::llvm::{expected_got_error};
use crate::llvm::{identifier_from_parse_tree, type_from_parse_tree, arguments_from_parse_tree};

use crate::cli::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode
{
    Ret
}

#[derive(Debug, Clone)]
pub struct Symbol
{
    title: String,
    datatype: DataType
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

#[derive(Debug, Clone, Copy)]
pub struct Literal
{
    value: i128,
    datatype: DataType
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

#[derive(Debug, Clone)]
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
    opcode: OpCode,
    arguments: Vec<Value>
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
        write!(f, "{:?}  ", self.opcode)?;

        for arg in &self.arguments
        {
            write!(f, "{:20}", arg)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Function
{
    pub instructions: HashMap<usize, Instruction>,
    pub labels: HashMap<usize, Vec<String>>,

    symbol_table: HashMap<String, Symbol>,

    return_type: DataType,
    name: String,
    arguments: Vec<(String, DataType)>,

    next_label: usize,
    next_register: usize,
    next_index: usize
}

impl Function
{
    pub fn new() -> Self
    {
        Self
        {
            instructions: HashMap::new(),
            labels: HashMap::new(),

            symbol_table: HashMap::new(),

            return_type: DataType::new(NonPtrType::Void, 0),
            name: String::from("[UNKNOWN]"),
            arguments: vec![],

            next_label: 0,
            next_register: 0,
            next_index: 0
        }
    }

    pub fn from_parse_tree_node(node: ParseTreeNode) -> Result<Self, Error>
    {
        let mut result = Self::new();

        match node
        {
            ParseTreeNode::Function(children) =>
            {
                let name = identifier_from_parse_tree(children[1].clone())?;
                let return_type = type_from_parse_tree(children[0].clone())?;
                let arguments = arguments_from_parse_tree(children[2].clone())?;

                result.set_function_signature(return_type, name, arguments);
            },
            default =>
            {
                expected_got_error("Function", default)?;
            }
        }

        Ok(result)
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

    pub fn place_label(&mut self, label: String, index: usize)
    {
        if self.labels.contains_key(&index)
        {
            self.labels.get_mut(&index).unwrap().push(label);
        }
        else
        {
            self.labels.insert(index, vec![label]);
        }
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

        for i in 0..self.next_index
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

            writeln!(f, "{}", self.instructions.get(&i).unwrap())?;
        }

        Ok(())
    }
}
