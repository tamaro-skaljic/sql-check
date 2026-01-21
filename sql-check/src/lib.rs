//! # SQL Check
//!
//! Compile-time SQL validation extracted from SQLx.
//!
//! This crate provides a `check!` macro that validates SQL queries at compile time
//! by connecting to a database and ensuring the query is valid.
//!
//! ## Features
//!
//! - `check` (default): Enables compile-time SQL validation
//!   - When disabled, the macro becomes a no-op and just returns the SQL string
//! - `postgres`: PostgreSQL support
//! - `mysql`: MySQL support  
//! - `sqlite`: SQLite support
//!
//! ## Example
//!
//! ```ignore
//! use sql_check::check;
//!
//! // This will be validated at compile time if the `check` feature is enabled
//! let sql = check!("SELECT 1");
//! ```

#![deny(rust_2018_idioms)]

// When the `check` feature is enabled, delegate to the proc macro
#[cfg(feature = "check")]
#[doc(hidden)]
#[doc = "Internal implementation detail. Do not use directly. Use the `check!` macro instead."]
pub use sql_check_derive::check_impl;

/// Validates SQL at compile time and expands to the SQL string literal.
///
/// When the `check` feature is enabled (default), this macro:
/// 1. Connects to the database specified by `DATABASE_URL` environment variable
/// 2. Sends a prepare command to validate the SQL syntax and semantics
/// 3. If validation succeeds, expands to the SQL string literal
/// 4. If validation fails, produces a compilation error
///
/// When the `check` feature is disabled, this macro simply expands to the
/// SQL string literal without any validation (useful for production builds).
///
/// # Examples
///
/// ```ignore
/// use sql_check::check;
///
/// // Valid SQL - compiles successfully
/// let sql = check!("SELECT 1");
///
/// // Invalid SQL - fails at compile time (when check feature is enabled)
/// // let sql = check!("SELECT invalid_column FROM nonexistent_table");
/// ```
#[cfg(feature = "check")]
#[macro_export]
macro_rules! check {
    ($sql:expr) => {
        $crate::check_impl!($sql)
    };
}

/// Validates SQL at compile time and expands to the SQL string literal.
///
/// When the `check` feature is disabled, this is a no-op that just
/// returns the SQL string literal.
#[cfg(not(feature = "check"))]
#[macro_export]
macro_rules! check {
    ($sql:expr) => {
        $sql
    };
}
