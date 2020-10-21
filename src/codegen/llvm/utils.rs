use crate::irgen::{DataType, NonPtrType};

/// Convert a type to a string in the format llvm uses (no u32 or u64, just i32, i64 etc.)
pub fn convert_to_llvm(datatype: &DataType) -> String
{
    format!("{}", datatype).replace("u", "i")
}

/// Gets the number of bytes in a type
pub fn bytes_size_of(datatype: &DataType) -> usize
{
    if datatype.num_ptr > 0
    {
        8
    }
    else
    {
        match datatype.raw_type
        {
            NonPtrType::I8 | NonPtrType::U8 => 1,
            NonPtrType::I16 | NonPtrType::U16 => 2,
            NonPtrType::I32 | NonPtrType::U32 => 4,
            NonPtrType::I64 | NonPtrType::U64 => 8,
            NonPtrType::Void => 0,
            NonPtrType::Bool => 1,
            NonPtrType::Unknown => {panic!()}
        }
    }
}