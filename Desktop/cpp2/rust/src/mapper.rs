

pub enum Int {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

impl Int {
    pub fn to_cpp(&self) -> &str {
        match self {
            Int::U8 => "uint_8t",
            Int::U16 => "uint_16t",
            Int::U32 => "uint_32t",
            Int::U64 => "uint_64t",
            _ => "unimplemented type"
        }
    }
}

pub enum Type {
    Int(Int),
    Bool
}
