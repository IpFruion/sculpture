use pest::Parser as OtherParser;
use pest_derive::Parser;

use crate::Error;
use crate::Sculptable;

#[derive(Parser)]
#[grammar = "grammar/rust.pest"]
struct Parser;

impl<'a> Sculptable for &'a str {
    type Input = Self;

    fn sculpt<S: crate::Sculptor>(
        _sculptor: &mut S,
        input: Self::Input,
    ) -> Result<S::Ok, S::Error> {
        let mut file = Parser::parse(Rule::file, input).map_err(S::Error::custom)?;
        let latest = Ok(S::Ok::default());
        while let Some(p) = file.next().filter(|p| matches!(p.as_rule(), Rule::outer)) {
            println!("{:?}", p);
            // let mut pairs = p.into_inner();
            // let vis = pairs
            //     .next()
            //     .ok_or_else(|| S::Error::custom("couldn't find visibility modifier"))?;
        }
        latest
    }
}

#[cfg(test)]
mod tests {
    use crate::{modifier::Modifier, tests::TestSculptor, SelfSculptable};

    #[test]
    fn single_empty_struct() {
        let mut sculptor = TestSculptor::default();
        let data = r#"pub struct MyStruct;"#;
        data.sculpt_self(&mut sculptor).unwrap();

        assert!(sculptor.starts.len() > 1);
        assert_eq!(
            sculptor.starts[0],
            (Modifier::Public, "MyStruct".to_owned())
        );
        assert!(sculptor.ends > 1);
    }
}
