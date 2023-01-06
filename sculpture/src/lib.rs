use err::Error;
use field_type::FieldType;
use modifier::Modifier;

pub mod err;
pub mod field_type;
pub mod modifier;
pub mod rust;
pub mod ts;

pub trait Sculptable {
    type Input;
    fn sculpt<S: Sculptor>(sculptor: &mut S, input: Self::Input) -> Result<S::Ok, S::Error>;
}

pub trait SculptableStruct {
    fn sculpt_struct<S: Sculptor>(sculptor: &mut S) -> Result<S::Ok, S::Error>;
}

impl<T: Sculptable<Input = ()>> SculptableStruct for T {
    fn sculpt_struct<S: Sculptor>(sculptor: &mut S) -> Result<S::Ok, S::Error> {
        T::sculpt(sculptor, ())
    }
}

pub trait Sculptor {
    type Ok;
    type Error: Error;

    fn start(&mut self, modifier: Modifier, name: &str) -> Result<Self::Ok, Self::Error>;
    fn field<'a>(
        &mut self,
        modifier: Modifier,
        name: &str,
        field_type: FieldType<'a>,
    ) -> Result<Self::Ok, Self::Error>;
    fn end(&mut self) -> Result<Self::Ok, Self::Error>;
}
