use std::fmt::Write;

use crate::err;
use crate::err::SculptureError;
use crate::field_type::FieldType;
use crate::modifier::Modifier;
use crate::Error;

pub mod parser;

pub struct Sculptor<W: Write> {
    writer: W,
}

impl<W: Write> Sculptor<W> {
    pub fn new(writer: W) -> Self {
        Sculptor { writer }
    }

    fn write_str(&mut self, s: &str) -> Result<(), SculptureError> {
        write!(self.writer, "{}", s).map_err(err::Error::custom)
    }

    fn write_field_type<'a>(&mut self, field_type: FieldType<'a>) -> Result<(), SculptureError> {
        match field_type {
            FieldType::Struct(s) => write!(self.writer, "{}", s).map_err(err::Error::custom),
            FieldType::Option(inner) => {
                self.write_str("Option<")?;
                self.write_field_type(*inner)?;
                self.write_str(">")
            }
            FieldType::Array(size, inner) => {
                self.write_str("[")?;
                self.write_field_type(*inner)?;
                write!(self.writer, "; {}]", size).map_err(err::Error::custom)
            }
            FieldType::String => self.write_str("String"),
            FieldType::Char => self.write_str("char"),
            FieldType::I8 => self.write_str("i8"),
            FieldType::I16 => self.write_str("i16"),
            FieldType::I32 => self.write_str("i32"),
            FieldType::I64 => self.write_str("i64"),
            FieldType::I128 => self.write_str("i128"),
            FieldType::U8 => self.write_str("u8"),
            FieldType::U16 => self.write_str("u16"),
            FieldType::U32 => self.write_str("u32"),
            FieldType::U64 => self.write_str("u64"),
            FieldType::U128 => self.write_str("u128"),
        }
    }

    fn write_modifier(&mut self, modifier: Modifier) -> Result<(), SculptureError> {
        match modifier {
            Modifier::Public => write!(self.writer, "pub ").map_err(err::Error::custom)?,
            Modifier::Private | Modifier::None => {}
        };
        Ok(())
    }
}

impl<W: Write> crate::Sculptor for Sculptor<W> {
    type Ok = ();
    type Error = err::SculptureError;

    fn start(&mut self, modifier: Modifier, name: &str) -> Result<Self::Ok, Self::Error> {
        self.write_modifier(modifier)?;
        writeln!(self.writer, "struct {}{{", name).map_err(Self::Error::custom)
    }

    fn field<'a>(
        &mut self,
        modifier: Modifier,
        name: &str,
        field_type: FieldType<'a>,
    ) -> Result<Self::Ok, Self::Error> {
        self.write_modifier(modifier)?;
        write!(self.writer, "\t{}: ", name).map_err(Self::Error::custom)?;
        self.write_field_type(field_type)?;
        writeln!(self.writer, ",").map_err(Self::Error::custom)
    }

    fn end(&mut self) -> Result<Self::Ok, Self::Error> {
        writeln!(self.writer, "}}").map_err(Self::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::{modifier::Modifier, Sculptable, StructScuptable};

    use super::Sculptor;

    #[test]
    fn empty_structure() {
        struct MyStruct;

        impl Sculptable for MyStruct {
            type Input = ();

            fn sculpt<S: crate::Sculptor>(
                sculptor: &mut S,
                _: Self::Input,
            ) -> Result<S::Ok, S::Error> {
                sculptor.start(Modifier::None, "MyStruct")?;
                sculptor.end()
            }
        }

        let mut output = String::new();
        let mut sculptor = Sculptor {
            writer: &mut output,
        };

        MyStruct::sculpt_struct(&mut sculptor).unwrap();
        println!("{}", output);
    }
}
