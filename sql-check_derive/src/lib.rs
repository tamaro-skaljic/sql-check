use proc_macro::TokenStream;
use quote::quote;

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
mod database;
#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
mod runtime;

/// Internal procedural macro for SQL validation.
/// Do not use directly - use the `check!` macro from the `sql-check` crate instead.
#[proc_macro]
pub fn check_impl(input: TokenStream) -> TokenStream {
    let input_sql = syn::parse_macro_input!(input as syn::LitStr);
    let sql_value = input_sql.value();

    // Get DATABASE_URL
    let database_url = match env("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => {
            // If DATABASE_URL is not set, emit a compile error
            return quote! {
                compile_error!(#e)
            }
            .into();
        }
    };

    // Perform SQL validation
    match validate_sql(&sql_value, &database_url) {
        Ok(()) => {
            // Validation succeeded, return the SQL string
            quote! { #input_sql }.into()
        }
        Err(e) => {
            // Validation failed, emit a compile error
            let error_msg = format!("SQL validation failed: {}", e);
            quote! {
                compile_error!(#error_msg)
            }
            .into()
        }
    }
}

fn validate_sql(sql: &str, database_url: &str) -> Result<(), String> {
    // Parse URL to determine database type
    // Handle both uppercase and lowercase schemes
    let url_lower = database_url.to_lowercase();

    if url_lower.starts_with("postgres://") || url_lower.starts_with("postgresql://") {
        validate_postgres(sql, database_url)
    } else if url_lower.starts_with("mysql://") {
        validate_mysql(sql, database_url)
    } else if url_lower.starts_with("sqlite://") || url_lower.starts_with("sqlite:") {
        validate_sqlite(sql, database_url)
    } else {
        Err(format!("Unsupported database URL scheme. Expected postgres://, mysql://, or sqlite://. Got: {}", 
            database_url.split("://").next().unwrap_or("unknown")))
    }
}

#[cfg(feature = "postgres")]
fn validate_postgres(sql: &str, database_url: &str) -> Result<(), String> {
    use sqlx_core::config;

    let driver_config = config::drivers::Config::default();

    <sqlx_postgres::Postgres as database::DatabaseExt>::describe_blocking(
        sql,
        database_url,
        &driver_config,
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

#[cfg(not(feature = "postgres"))]
fn validate_postgres(_sql: &str, _database_url: &str) -> Result<(), String> {
    Err("PostgreSQL support not enabled. Enable the 'postgres' feature.".to_string())
}

#[cfg(feature = "mysql")]
fn validate_mysql(sql: &str, database_url: &str) -> Result<(), String> {
    use sqlx_core::config;

    let driver_config = config::drivers::Config::default();

    <sqlx_mysql::MySql as database::DatabaseExt>::describe_blocking(
        sql,
        database_url,
        &driver_config,
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

#[cfg(not(feature = "mysql"))]
fn validate_mysql(_sql: &str, _database_url: &str) -> Result<(), String> {
    Err("MySQL support not enabled. Enable the 'mysql' feature.".to_string())
}

#[cfg(feature = "sqlite")]
fn validate_sqlite(sql: &str, database_url: &str) -> Result<(), String> {
    use sqlx_core::config;

    let driver_config = config::drivers::Config::default();

    <sqlx_sqlite::Sqlite as database::DatabaseExt>::describe_blocking(
        sql,
        database_url,
        &driver_config,
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

#[cfg(not(feature = "sqlite"))]
fn validate_sqlite(_sql: &str, _database_url: &str) -> Result<(), String> {
    Err("SQLite support not enabled. Enable the 'sqlite' feature.".to_string())
}

fn env(var: &str) -> Result<String, String> {
    std::env::var(var).map_err(|_| {
        format!(
            "Environment variable {} must be set to use SQL validation",
            var
        )
    })
}
