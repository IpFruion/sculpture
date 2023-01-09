use pest::iterators::Pair;
use pest::Parser as OtherParser;
use pest_derive::Parser;

use crate::field_type::FieldType;
use crate::modifier::Modifier;
use crate::Error;
use crate::Sculptable;

#[derive(Parser)]
#[grammar = "grammar/rust.pest"]
struct Parser;

fn into_modifier(p: Pair<Rule>) -> Option<Modifier> {
    if !matches!(p.as_rule(), Rule::vis) {
        return None;
    }

    Some(match p.as_str() {
        "pub" => Modifier::Public,
        _ => Modifier::None,
    })
}

fn into_identity<'a>(p: Pair<'a, Rule>) -> Option<&'a str> {
    if !matches!(p.as_rule(), Rule::ident) {
        return None;
    }
    Some(p.as_str())
}

fn into_field_type<'a>(p: Pair<'a, Rule>) -> Option<FieldType<'a>> {
    if !matches!(p.as_rule(), Rule::ident) {
        return None;
    }
    Some(match p.as_str() {
        "String" => FieldType::String,
        "i8" => FieldType::I8,
        "i16" => FieldType::I16,
        "i32" => FieldType::I32,
        "i64" => FieldType::I64,
        "i128" => FieldType::I128,
        "u8" => FieldType::U8,
        "u16" => FieldType::U16,
        "u32" => FieldType::U32,
        "u64" => FieldType::U64,
        "u128" => FieldType::U128,
        k => FieldType::Struct(k),
    })
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
            let modifier = pairs
                .next()
                .and_then(into_modifier)
                .expect("couldn't find visibility modifier");
            let ident = pairs
                .next()
                .and_then(into_identity)
                .expect("couldn't find struct identifier");

            sculptor.start(modifier, ident)?;

            while let Some(field) = pairs.next() {
                let mut pieces = field.into_inner();
                let modifier = pieces
                    .next()
                    .and_then(into_modifier)
                    .expect("couldn't find field visibility modifier");
                let name = pieces
                    .next()
                    .and_then(into_identity)
                    .expect("couldn't find field identifier");
                let field_type = pieces
                    .next()
                    .and_then(into_field_type)
                    .expect("couldn't find field type");

                sculptor.field(modifier, name, field_type)?;
            }
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

    #[test]
    fn single_field_struct() {
        let mut sculptor = MockSculptor::default();
        let data = r#"pub struct MyStruct{ thing: String, }"#;
        data.sculpt_self(&mut sculptor).unwrap();

        assert_eq!(sculptor.starts.len(), 1);
        assert_eq!(
            sculptor.starts[0],
            (Modifier::Public, "MyStruct".to_owned())
        );

        assert_eq!(sculptor.fields.len(), 1);
        assert_eq!(
            sculptor.fields[0],
            (Modifier::None, "thing".to_owned(), "String".to_owned())
        );
        assert_eq!(sculptor.ends, 1);
    }
}
