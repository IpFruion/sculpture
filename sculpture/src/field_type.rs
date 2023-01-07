//TODO: possibly function definitions?
pub enum FieldType<'a> {
    Struct(&'a str),
    Option(Box<FieldType<'a>>),
    Array(usize, Box<FieldType<'a>>),
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    Char,
    String,
}
