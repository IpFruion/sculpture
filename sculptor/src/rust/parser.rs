use pest::iterators::Pair;
use pest::Parser as OtherParser;
use pest_derive::Parser;

use crate::modifier::Modifier;
use crate::Error;
use crate::Sculptable;

#[derive(Parser)]
#[grammar = "grammar/rust.pest"]
struct Parser;

fn into_modifier(p: Pair<Rule>) -> Modifier {
    match p.as_str() {
        "pub" => Modifier::Public,
        _ => Modifier::None,
    }
}

/// Implementation of Scultable for any arbirary `&str` for Rust.
///
/// Note that this panics if the grammar is not set up properly.
impl<'a> Sculptable for &'a str {
    type Input = Self;

    fn sculpt<S: crate::Sculptor>(sculptor: &mut S, input: Self::Input) -> Result<S::Ok, S::Error> {
        let mut file = Parser::parse(Rule::file, input).map_err(S::Error::custom)?;

        //Parsing empty file results in an ok status because it was empty so no actions were
        //performed.
        let mut latest = S::Ok::default();

        while let Some(p) = file.next().filter(|p| matches!(p.as_rule(), Rule::outer)) {
            println!("{:?}", p);
            let mut pairs = p.into_inner();
            let vis = pairs.next().expect("couldn't find visibility modifier");
            let modifier = into_modifier(vis);
            let ident = pairs
                .next()
                .expect("couldn't find struct identifier")
                .as_str();
            println!("{:?}, {}", modifier, ident);
            sculptor.start(modifier, ident)?;

            while let Some(_field) = pairs.next() {}
            latest = sculptor.end()?;
        }
        Ok(latest)
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::MockSculptor;
    use crate::{modifier::Modifier, SelfSculptable};

    #[test]
    fn single_empty_struct() {
        let mut sculptor = MockSculptor::default();
        let data = r#"pub struct MyStruct;"#;
        data.sculpt_self(&mut sculptor).unwrap();

        assert_eq!(sculptor.starts.len(), 1);
        assert_eq!(
            sculptor.starts[0],
            (Modifier::Public, "MyStruct".to_owned())
        );
        assert_eq!(sculptor.ends, 1);
    }
}
