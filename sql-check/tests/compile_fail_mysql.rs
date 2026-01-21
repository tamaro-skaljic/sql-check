#[cfg(all(feature = "check", feature = "mysql"))]
#[test]
fn compile_fail_tests_mysql() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/compile_fail/mysql/*.rs");
}
