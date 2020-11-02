use std::fmt;

/// Non Pointer Type, a raw type
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum NonPtrType
{
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Void,
    Bool,
    Unknown
}

/// A datatype with the possibility of being a pointer and a reference
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DataType
{
    pub raw_type: NonPtrType,
    pub num_ptr: usize,
    pub is_ref: bool
}

impl DataType
{
    /// Generate a new datatype object
    pub fn new(raw: NonPtrType, ptrs: usize, is_ref: bool) -> Self
    {
        Self
        {
            raw_type: raw,
            num_ptr: ptrs,
            is_ref: is_ref
        }
    }

    /// Is the datatype signed
    pub fn is_signed(&self) -> bool
    {
        if self.num_ptr > 0 || self.is_ref
        {
            false
        }
        else
        {
            match self.raw_type
            {
                NonPtrType::I8 | NonPtrType::I16 | NonPtrType::I32 | NonPtrType::I64 => true,
                _ => false
            }
        }   
    }
}

impl fmt::Display for DataType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        if self.is_ref
        {
            write!(f, "&")?;
        }
        
        write!(f, "{}", match self.raw_type
        {
            NonPtrType::Bool => "i1",
            NonPtrType::I8 => "i8",
            NonPtrType::U8 => "u8",
            NonPtrType::I16 => "i16",
            NonPtrType::U16 => "u16",
            NonPtrType::I32 => "i32",
            NonPtrType::U32 => "u32",
            NonPtrType::I64 => "i64",
            NonPtrType::U64 => "u64",
            NonPtrType::Void => "void",
            NonPtrType::Unknown => "Unk"
        })?;

        for _ in 0..self.num_ptr
        {
            write!(f, "*")?;
        }

        Ok(())
    }
}