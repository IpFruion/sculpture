#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/empty_struct.rs");
}
