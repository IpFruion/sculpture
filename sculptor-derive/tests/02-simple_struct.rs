use sculptor::mocks::MockSculptor;
use sculptor::StructSculptable;
use sculptor_derive::Sculptable;

#[derive(Sculptable)]
struct MyStruct {
    thing: String,
}

fn main() {
    let mut sculptor = MockSculptor::default();
    MyStruct::sculpt_struct(&mut sculptor).unwrap();

    assert_eq!(sculptor.starts.len(), 1);
    assert_eq!(
        sculptor.starts[0],
        (sculptor::modifier::Modifier::None, "MyStruct".to_owned())
    );
    assert_eq!(sculptor.fields.len(), 1);
    assert_eq!(
        sculptor.fields[0],
        (
            sculptor::modifier::Modifier::None,
            "thing".to_owned(),
            "String".to_owned(),
        )
    );
    assert_eq!(sculptor.ends, 1);
}
