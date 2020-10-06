use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy)]
pub struct DataType
{
    pub raw_type: NonPtrType,
    pub num_ptr: usize
}

impl DataType
{
    pub fn new(raw: NonPtrType, ptrs: usize) -> Self
    {
        Self
        {
            raw_type: raw,
            num_ptr: ptrs
        }
    }
}

impl fmt::Display for DataType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
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