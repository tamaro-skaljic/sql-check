//! Example of using sql-check with SQLite
//!
//! This example demonstrates how to use the check! macro to validate SQL queries
//! at compile time before using them with your database library.
//!
//! Prerequisites:
//! - DATABASE_URL environment variable set to a SQLite database
//!
//! Run with:
//! ```bash
//! DATABASE_URL="sqlite::memory:" cargo run --example sqlite --features sqlite,_rt-tokio,_tls-rustls-ring-webpki
//! ```

use sql_check::check;

fn main() {
    println!("SQL Check - SQLite Example");
    println!("===========================\n");

    // Valid SQL queries - these will be validated at compile time
    let select_query = check!("SELECT 1");
    println!("✓ Valid SELECT query: {}", select_query);

    let select_with_function = check!("SELECT datetime('now')");
    println!("✓ Valid SELECT with function: {}", select_with_function);

    // SQLite-specific syntax validation
    let select_with_coalesce = check!("SELECT COALESCE(NULL, 'default')");
    println!("✓ Valid SELECT with COALESCE: {}", select_with_coalesce);

    // CREATE TABLE syntax validation
    let create_table = check!(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    );
    println!("✓ Valid CREATE TABLE query: {}", create_table);

    // DML queries (requires 'users' table to exist in the database at compile time, see .github/workflows/test.yml for setup)
    let insert_query = check!("INSERT INTO users (name, email) VALUES (?, ?)");
    println!("✓ Valid INSERT query: {}", insert_query);

    let update_query = check!("UPDATE users SET name = ? WHERE id = ?");
    println!("✓ Valid UPDATE query: {}", update_query);

    let delete_query = check!("DELETE FROM users WHERE id = ?");
    println!("✓ Valid DELETE query: {}", delete_query);

    println!("\n✅ All SQL queries validated successfully at compile time!");
    println!("   You can now use these SQL strings with any SQLite client library.");

    // Example: The following would cause a compile-time error
    // Uncomment to see the error:
    // let invalid = check!("SELECT * FROM nonexistent_table");
    // let syntax_error = check!("SELECT FROM WHERE");
}
