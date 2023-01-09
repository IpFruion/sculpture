use sculptor::mocks::MockSculptor;
use sculptor::StructSculptable;
use sculptor_derive::Sculptable;

#[derive(Sculptable)]
pub struct MyStruct;

fn main() {
    let mut sculptor = MockSculptor::default();
    MyStruct::sculpt_struct(&mut sculptor).unwrap();

    assert_eq!(sculptor.starts.len(), 1);
    assert_eq!(
        sculptor.starts[0],
        (sculptor::modifier::Modifier::Public, "MyStruct".to_owned())
    );
    assert_eq!(sculptor.ends, 1);
}
