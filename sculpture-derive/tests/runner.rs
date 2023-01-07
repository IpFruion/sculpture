#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-empty_struct.rs");
    t.pass("tests/02-simple_struct.rs");
}
