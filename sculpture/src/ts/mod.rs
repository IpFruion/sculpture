use crate::err::Error;
use std::fmt::Write;

use crate::{err, field_type::FieldType, modifier::Modifier};

pub struct Sculptor<W: Write> {
    writer: W,
}

impl<W: Write> Sculptor<W> {
    fn write_field_type<'a>(
        &mut self,
        field_type: FieldType<'a>,
    ) -> Result<(), err::SculptureError> {
        match field_type {
            FieldType::Char => write!(self.writer, "char").map_err(err::Error::custom),
            FieldType::String => write!(self.writer, "string").map_err(err::Error::custom),
            FieldType::Struct(s) => write!(self.writer, "{}", s).map_err(err::Error::custom),
            FieldType::Option(inner) => self.write_field_type(*inner),
            FieldType::Array(_, inner) => {
                write!(self.writer, "Array<").map_err(err::Error::custom)?;
                self.write_field_type(*inner)?;
                write!(self.writer, ">").map_err(err::Error::custom)
            }
            FieldType::I8
            | FieldType::I16
            | FieldType::I32
            | FieldType::I64
            | FieldType::I128
            | FieldType::U8
            | FieldType::U16
            | FieldType::U32
            | FieldType::U64
            | FieldType::U128 => write!(self.writer, "number").map_err(err::Error::custom),
        }
    }

    fn write_modifier(&mut self, modifier: Modifier) -> Result<(), err::SculptureError> {
        match modifier {
            Modifier::Public => write!(self.writer, "public"),
            Modifier::Private => write!(self.writer, "private"),
            Modifier::None => Ok(()),
        }
        .map_err(err::Error::custom)
    }
}

impl<W: Write> crate::Sculptor for Sculptor<W> {
    type Ok = ();
    type Error = err::SculptureError;

    fn start(&mut self, modifier: Modifier, name: &str) -> Result<Self::Ok, Self::Error> {
        self.write_modifier(modifier)?;
        writeln!(self.writer, "class {} {{", name).map_err(Self::Error::custom)
    }

    fn field<'a>(
        &mut self,
        modifier: Modifier,
        name: &str,
        field_type: FieldType<'a>,
    ) -> Result<Self::Ok, Self::Error> {
        write!(self.writer, "\t").map_err(Self::Error::custom)?;
        self.write_modifier(modifier)?;
        write!(self.writer, " {}", name).map_err(Self::Error::custom)?;
        if matches!(field_type, FieldType::Option(_)) {
            write!(self.writer, "?").map_err(Self::Error::custom)?;
        }
        write!(self.writer, ": ").map_err(Self::Error::custom)?;
        self.write_field_type(field_type)?;
        writeln!(self.writer, ";").map_err(Self::Error::custom)
    }

    fn end(&mut self) -> Result<Self::Ok, Self::Error> {
        writeln!(self.writer, "}}").map_err(Self::Error::custom)
    }
}
