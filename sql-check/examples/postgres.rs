//! Example of using sql-check with PostgreSQL
//!
//! This example demonstrates how to use the check! macro to validate SQL queries
//! at compile time before using them with your database library.
//!
//! Prerequisites:
//! - PostgreSQL database running
//! - DATABASE_URL environment variable set
//!
//! Run with:
//! ```bash
//! DATABASE_URL="postgres://user:pass@localhost/mydb" cargo run --example postgres --features postgres,_rt-tokio,_tls-rustls-ring-webpki
//! ```

use sql_check::check;

fn main() {
    println!("SQL Check - PostgreSQL Example");
    println!("================================\n");

    // Valid SQL queries - these will be validated at compile time
    let select_query = check!("SELECT 1");
    println!("✓ Valid SELECT query: {}", select_query);

    let select_with_column = check!("SELECT current_timestamp");
    println!("✓ Valid SELECT with function: {}", select_with_column);

    // More complex queries
    let create_table = check!(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )"
    );
    println!("✓ Valid CREATE TABLE query validated");

    let insert_query = check!(
        "INSERT INTO users (name, email) VALUES ($1, $2)"
    );
    println!("✓ Valid INSERT query: {}", insert_query);

    let update_query = check!(
        "UPDATE users SET name = $1 WHERE id = $2"
    );
    println!("✓ Valid UPDATE query: {}", update_query);

    let delete_query = check!(
        "DELETE FROM users WHERE id = $1"
    );
    println!("✓ Valid DELETE query: {}", delete_query);

    println!("\n✅ All SQL queries validated successfully at compile time!");
    println!("   You can now use these SQL strings with any PostgreSQL client library.");

    // Example: The following would cause a compile-time error
    // Uncomment to see the error:
    // let invalid = check!("SELECT * FROM nonexistent_table");
    // let syntax_error = check!("SELECT FROM WHERE");
}
