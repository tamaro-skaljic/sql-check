//! Tests for the sql-check crate with the `check` feature enabled and disabled

#[cfg(feature = "check")]
#[test]
fn test_check_feature_enabled_returns_string() {
    use sql_check::check;
    
    let sql = check!("SELECT 1");
    assert_eq!(sql, "SELECT 1");
}

#[cfg(feature = "check")]
#[test]
fn test_check_validates_multiple_select_statements() {
    use sql_check::check;

    let select_integer = check!("SELECT 1");
    let select_string = check!("SELECT 'hello'");
    let select_arithmetic = check!("SELECT 1 + 2");

    assert_eq!(select_integer, "SELECT 1");
    assert_eq!(select_string, "SELECT 'hello'");
    assert_eq!(select_arithmetic, "SELECT 1 + 2");
}

#[cfg(not(feature = "check"))]
#[test]
fn test_check_feature_disabled_returns_string() {
    use sql_check::check;
    
    let sql = check!("SELECT 1");
    assert_eq!(sql, "SELECT 1");
}

#[cfg(not(feature = "check"))]
#[test]
fn test_check_feature_disabled_no_validation() {
    use sql_check::check;
    
    // When check feature is disabled, even invalid SQL should compile
    // (no database connection attempted)
    let sql = check!("INVALID SQL SYNTAX");
    assert_eq!(sql, "INVALID SQL SYNTAX");
}
