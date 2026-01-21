#[cfg(all(feature = "check", feature = "sqlite"))]
#[test]
fn compile_fail_tests_sqlite() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/compile_fail/sqlite/*.rs");
}
