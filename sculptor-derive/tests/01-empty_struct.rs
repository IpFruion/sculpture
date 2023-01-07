use sculpture::rust;
use sculpture::Sculptable as OtherSculptable;
use sculpture_derive::Sculptable;

#[derive(Sculptable)]
pub struct MyStruct;

fn main() {
    let mut output = String::new();
    let mut sculptor = rust::Sculptor::new(&mut output);
    MyStruct::sculpt(&mut sculptor, ()).unwrap();
    assert_eq!("pub struct MyStruct{\n}\n", output)
}
