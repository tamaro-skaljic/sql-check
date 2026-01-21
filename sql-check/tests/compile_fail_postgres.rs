#[cfg(all(feature = "check", feature = "postgres"))]
#[test]
fn compile_fail_tests_postgres() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/compile_fail/postgres/*.rs");
}
