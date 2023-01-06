use std::marker::PhantomData;

use pest::Parser as OtherParser;
use pest_derive::Parser;

use crate::Error;
use crate::Sculptable;

#[derive(Parser)]
#[grammar = "grammar/rust.pest"]
struct Parser;

pub struct StrSculptable<'a>(PhantomData<&'a ()>);

impl<'a> Sculptable for StrSculptable<'a> {
    type Input = &'a str;
    fn sculpt<S: crate::Sculptor>(sculptor: &mut S, input: Self::Input) -> Result<S::Ok, S::Error> {
        let mut file = Parser::parse(Rule::file, input).map_err(S::Error::custom)?;
        println!("{:?}", file);
        let _top = file
            .next()
            .ok_or_else(|| S::Error::custom("unable to find top"))?;
        sculptor.end()
    }
}

#[cfg(test)]
mod tests {}
